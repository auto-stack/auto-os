// AutoIME - composition/edit planning helpers (PLAN-070 T-02).
#include "edit_model.h"

#include <windows.h>

namespace autoime {

std::wstring Utf8ToUtf16(const std::string& utf8) {
  if (utf8.empty()) return std::wstring();
  const int n = MultiByteToWideChar(CP_UTF8, 0, utf8.c_str(),
                                    static_cast<int>(utf8.size()), nullptr, 0);
  if (n <= 0) return std::wstring();
  std::wstring out(static_cast<size_t>(n), L'\0');
  MultiByteToWideChar(CP_UTF8, 0, utf8.c_str(), static_cast<int>(utf8.size()),
                      &out[0], n);
  return out;
}

int Utf16OffsetFromUtf8Codepoint(const std::string& utf8,
                                    int codepoint_offset) {
  if (codepoint_offset <= 0) return 0;
  int codepoints = 0;
  int utf16 = 0;
  size_t i = 0;
  while (i < utf8.size()) {
    if (codepoints == codepoint_offset) return utf16;
    const unsigned char c = static_cast<unsigned char>(utf8[i]);
    size_t bytes = 1;
    if ((c & 0x80) == 0)
      bytes = 1;
    else if ((c & 0xE0) == 0xC0)
      bytes = 2;
    else if ((c & 0xF0) == 0xE0)
      bytes = 3;
    else if ((c & 0xF8) == 0xF0)
      bytes = 4;
    if (i + bytes > utf8.size()) bytes = utf8.size() - i;
    utf16 += (bytes == 4) ? 2 : 1;
    i += bytes;
    ++codepoints;
  }
  return utf16;
}

EditAction PlanCompositionApply(bool has_commit, bool commit_empty,
                                bool preedit_empty) {
  const bool commit = has_commit && !commit_empty;
  if (commit && !preedit_empty) return EditAction::kCommitThenCompose;
  if (commit) return EditAction::kCommit;
  if (!preedit_empty) return EditAction::kCompose;
  return EditAction::kRestore;
}

}  // namespace autoime
