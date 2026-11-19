// Minimal JSON reader for AutoIME shared test cases (PLAN-071 T-01).
//
// The shared cases file (shared/tests/cases.json) is plain UTF-8 JSON with
// objects, arrays, strings, numbers and booleans. This parser covers exactly
// that subset - no external dependency, so Windows and a future Linux replay
// can consume the same file with the same rules. Parse failures are reported
// through the error out-parameter; nothing throws or aborts.
#ifndef AUTOIME_JSON_MINI_H_
#define AUTOIME_JSON_MINI_H_

#include <cstdio>
#include <memory>
#include <string>
#include <utility>
#include <vector>

namespace json_mini {

struct Value;
using ValuePtr = std::unique_ptr<Value>;

struct Value {
  enum class Kind { Null, Bool, Number, String, Array, Object };
  Kind kind = Kind::Null;
  bool boolean = false;
  double number = 0.0;
  std::string string;
  std::vector<ValuePtr> array;
  std::vector<std::pair<std::string, ValuePtr>> object;

  const Value* member(const char* key) const {
    if (kind != Kind::Object) return nullptr;
    for (const auto& kv : object)
      if (kv.first == key) return kv.second.get();
    return nullptr;
  }
  std::string str_or(const char* key, const std::string& fallback) const {
    const Value* v = member(key);
    return (v && v->kind == Kind::String) ? v->string : fallback;
  }
};

namespace detail {

inline void SkipWs(const std::string& s, size_t* pos) {
  while (*pos < s.size()) {
    const char c = s[*pos];
    if (c == ' ' || c == '\t' || c == '\r' || c == '\n') {
      ++*pos;
      continue;
    }
    break;
  }
}

inline void AppendUtf8(std::string* out, unsigned int cp) {
  if (cp < 0x80) {
    out->push_back(static_cast<char>(cp));
  } else if (cp < 0x800) {
    out->push_back(static_cast<char>(0xC0 | (cp >> 6)));
    out->push_back(static_cast<char>(0x80 | (cp & 0x3F)));
  } else if (cp < 0x10000) {
    out->push_back(static_cast<char>(0xE0 | (cp >> 12)));
    out->push_back(static_cast<char>(0x80 | ((cp >> 6) & 0x3F)));
    out->push_back(static_cast<char>(0x80 | (cp & 0x3F)));
  } else {
    out->push_back(static_cast<char>(0xF0 | (cp >> 18)));
    out->push_back(static_cast<char>(0x80 | ((cp >> 12) & 0x3F)));
    out->push_back(static_cast<char>(0x80 | ((cp >> 6) & 0x3F)));
    out->push_back(static_cast<char>(0x80 | (cp & 0x3F)));
  }
}

inline bool ParseHex4(const std::string& s, size_t* pos, unsigned int* out) {
  unsigned int value = 0;
  for (int i = 0; i < 4; ++i) {
    if (*pos >= s.size()) return false;
    const char c = s[(*pos)++];
    value <<= 4;
    if (c >= '0' && c <= '9') {
      value |= static_cast<unsigned int>(c - '0');
    } else if (c >= 'a' && c <= 'f') {
      value |= static_cast<unsigned int>(c - 'a' + 10);
    } else if (c >= 'A' && c <= 'F') {
      value |= static_cast<unsigned int>(c - 'A' + 10);
    } else {
      return false;
    }
  }
  *out = value;
  return true;
}

inline bool ParseStringRaw(const std::string& s, size_t* pos, std::string* out,
                           std::string* error) {
  if (*pos >= s.size() || s[*pos] != '"') {
    *error = "expected string";
    return false;
  }
  ++*pos;
  while (*pos < s.size()) {
    const char c = s[(*pos)++];
    if (c == '"') return true;
    if (c != '\\') {
      out->push_back(c);
      continue;
    }
    if (*pos >= s.size()) break;
    const char esc = s[(*pos)++];
    switch (esc) {
      case '"': out->push_back('"'); break;
      case '\\': out->push_back('\\'); break;
      case '/': out->push_back('/'); break;
      case 'b': out->push_back('\b'); break;
      case 'f': out->push_back('\f'); break;
      case 'n': out->push_back('\n'); break;
      case 'r': out->push_back('\r'); break;
      case 't': out->push_back('\t'); break;
      case 'u': {
        unsigned int cp = 0;
        if (!ParseHex4(s, pos, &cp)) {
          *error = "bad \\u escape";
          return false;
        }
        if (cp >= 0xD800 && cp <= 0xDBFF && *pos + 1 < s.size() &&
            s[*pos] == '\\' && s[*pos + 1] == 'u') {
          *pos += 2;
          unsigned int low = 0;
          if (!ParseHex4(s, pos, &low) || low < 0xDC00 || low > 0xDFFF) {
            *error = "bad surrogate pair";
            return false;
          }
          cp = 0x10000 + ((cp - 0xD800) << 10) + (low - 0xDC00);
        }
        AppendUtf8(out, cp);
        break;
      }
      default:
        *error = "unknown escape";
        return false;
    }
  }
  *error = "unterminated string";
  return false;
}

inline bool ParseValue(const std::string& s, size_t* pos, Value* out,
                       int depth, std::string* error);

inline bool ParseObject(const std::string& s, size_t* pos, Value* out,
                        int depth, std::string* error) {
  ++*pos;  // '{'
  while (true) {
    SkipWs(s, pos);
    if (*pos < s.size() && s[*pos] == '}') {
      ++*pos;
      return true;
    }
    std::string key;
    if (!ParseStringRaw(s, pos, &key, error)) return false;
    SkipWs(s, pos);
    if (*pos >= s.size() || s[*pos] != ':') {
      *error = "expected ':'";
      return false;
    }
    ++*pos;
    ValuePtr child(new Value());
    if (!ParseValue(s, pos, child.get(), depth + 1, error)) return false;
    out->object.emplace_back(std::move(key), std::move(child));
    SkipWs(s, pos);
    if (*pos < s.size() && s[*pos] == ',') {
      ++*pos;
      continue;
    }
    if (*pos < s.size() && s[*pos] == '}') {
      ++*pos;
      return true;
    }
    *error = "expected ',' or '}'";
    return false;
  }
}

inline bool ParseArray(const std::string& s, size_t* pos, Value* out,
                       int depth, std::string* error) {
  ++*pos;  // '['
  while (true) {
    SkipWs(s, pos);
    if (*pos < s.size() && s[*pos] == ']') {
      ++*pos;
      return true;
    }
    ValuePtr child(new Value());
    if (!ParseValue(s, pos, child.get(), depth + 1, error)) return false;
    out->array.push_back(std::move(child));
    SkipWs(s, pos);
    if (*pos < s.size() && s[*pos] == ',') {
      ++*pos;
      continue;
    }
    if (*pos < s.size() && s[*pos] == ']') {
      ++*pos;
      return true;
    }
    *error = "expected ',' or ']'";
    return false;
  }
}

inline bool ParseValue(const std::string& s, size_t* pos, Value* out,
                       int depth, std::string* error) {
  if (depth > 32) {
    *error = "nesting too deep";
    return false;
  }
  SkipWs(s, pos);
  if (*pos >= s.size()) {
    *error = "unexpected end of input";
    return false;
  }
  const char c = s[*pos];
  if (c == '{') {
    out->kind = Value::Kind::Object;
    return ParseObject(s, pos, out, depth, error);
  }
  if (c == '[') {
    out->kind = Value::Kind::Array;
    return ParseArray(s, pos, out, depth, error);
  }
  if (c == '"') {
    out->kind = Value::Kind::String;
    return ParseStringRaw(s, pos, &out->string, error);
  }
  if (c == 't' && s.compare(*pos, 4, "true") == 0) {
    *pos += 4;
    out->kind = Value::Kind::Bool;
    out->boolean = true;
    return true;
  }
  if (c == 'f' && s.compare(*pos, 5, "false") == 0) {
    *pos += 5;
    out->kind = Value::Kind::Bool;
    out->boolean = false;
    return true;
  }
  if (c == 'n' && s.compare(*pos, 4, "null") == 0) {
    *pos += 4;
    out->kind = Value::Kind::Null;
    return true;
  }
  // Number: JSON grammar subset with optional exponent.
  {
    const size_t start = *pos;
    if (c == '-' || c == '+') ++*pos;
    bool digits = false;
    while (*pos < s.size()) {
      const char d = s[*pos];
      if ((d >= '0' && d <= '9') || d == '.' || d == 'e' || d == 'E' ||
          d == '-' || d == '+') {
        if (d >= '0' && d <= '9') digits = true;
        ++*pos;
        continue;
      }
      break;
    }
    if (!digits) {
      *error = "invalid literal";
      *pos = start;
      return false;
    }
    out->kind = Value::Kind::Number;
    out->number = std::atof(s.c_str() + start);
    return true;
  }
}

}  // namespace detail

// Parses one JSON document. On failure returns false with a human-readable
// *error (offset included when known); *out is unspecified then.
inline bool Parse(const std::string& src, Value* out, std::string* error) {
  size_t pos = 0;
  error->clear();
  if (!detail::ParseValue(src, &pos, out, 0, error)) {
    char buf[32];
    std::snprintf(buf, sizeof(buf), " (offset %zu)", pos);
    *error += buf;
    return false;
  }
  detail::SkipWs(src, &pos);
  if (pos != src.size()) {
    *error = "trailing content after JSON document";
    return false;
  }
  return true;
}

}  // namespace json_mini

#endif  // AUTOIME_JSON_MINI_H_
