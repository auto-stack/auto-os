// AutoIME Windows x64 - T-01 engine probe (PLAN-069).
//
// Loads the Weasel-bundled x64 rime.dll, deploys the pinyin schema into a
// fresh EMPTY user directory, and re-verifies the PLAN-069 baseline:
//   t1  typing "nihao" composes "ni hao"; selecting candidate #1 commits
//       exactly one commit whose text is "你好"; the composition then clears.
//   t2  typing "ni" then Esc cancels the composition and commits nothing.
//   t3  two engine sessions keep composition and candidates isolated.
//
// Diagnosable failure modes (per T-01 completion conditions):
//   exit 2  rime.dll missing / not loadable (searched locations printed)
//   exit 3  schema file missing from the shared data dir (path printed)
//   exit 4  deployment failed or expected schema not current
//   exit 5  one of the behavioral tests failed (details printed)
//   exit 0  all tests passed
//
// Usage:
//   rime_probe.exe [--rime-dll PATH] [--shared-data-dir DIR]
//                  [--user-data-dir DIR] [--schema SCHEMA_ID] [--keep]
// Defaults resolve WeaselRoot from the registry (HKLM\...\Rime\Weasel) and
// create a fresh empty user dir under %TEMP% (kept when --keep is given;
// a probe-provided dir is removed at exit unless --keep).

#include "rime_api.h"

#include <windows.h>

#include <crtdbg.h>

#include <cstdio>
#include <cstdint>
#include <cstring>
#include <string>
#include <vector>

namespace {

constexpr int kKeysymEscape = 0xff1b;

struct Options {
  std::string rime_dll;
  std::string shared_data_dir;
  std::string user_data_dir;
  std::string schema = "luna_pinyin_simp";
  bool keep_user_dir = false;
  bool user_dir_from_temp = true;  // default user dir is probe-provided
};

// ---------------------------------------------------------------- utilities

std::string JoinPath(const std::string& a, const std::string& b) {
  if (a.empty()) return b;
  if (!b.empty() && (b[0] == '\\' || b[0] == '/')) return b;
  if (a.back() == '\\' || a.back() == '/') return a + b;
  return a + "\\" + b;
}

bool FileExists(const std::string& p) {
  DWORD attrs = GetFileAttributesA(p.c_str());
  return attrs != INVALID_FILE_ATTRIBUTES &&
         !(attrs & FILE_ATTRIBUTE_DIRECTORY);
}

bool ReadWeaselRootFromRegistry(std::string* out) {
  char buf[MAX_PATH]{};
  DWORD size = sizeof(buf);
  LSTATUS st = RegGetValueA(HKEY_LOCAL_MACHINE,
                            "SOFTWARE\\WOW6432Node\\Rime\\Weasel",
                            "WeaselRoot", RRF_RT_REG_SZ, nullptr, buf, &size);
  if (st != ERROR_SUCCESS) {
    size = sizeof(buf);
    st = RegGetValueA(HKEY_LOCAL_MACHINE, "SOFTWARE\\Rime\\Weasel",
                      "WeaselRoot", RRF_RT_REG_SZ, nullptr, buf, &size);
  }
  if (st != ERROR_SUCCESS || buf[0] == '\0') return false;
  *out = buf;
  return true;
}

std::string DllFileVersion(const std::string& path) {
  DWORD handle = 0;
  DWORD size = GetFileVersionInfoSizeA(path.c_str(), &handle);
  if (size == 0) return "n/a";
  std::vector<char> data(size);
  if (!GetFileVersionInfoA(path.c_str(), handle, size, data.data()))
    return "n/a";
  VS_FIXEDFILEINFO* info = nullptr;
  UINT len = 0;
  if (!VerQueryValueA(data.data(), "\\", reinterpret_cast<LPVOID*>(&info),
                      &len) || info == nullptr)
    return "n/a";
  char out[64];
  std::snprintf(out, sizeof(out), "%u.%u.%u.%u",
                HIWORD(info->dwFileVersionMS), LOWORD(info->dwFileVersionMS),
                HIWORD(info->dwFileVersionLS), LOWORD(info->dwFileVersionLS));
  return out;
}

bool RemoveTreeRecursively(const std::string& dir) {
  std::string pattern = JoinPath(dir, "*");
  WIN32_FIND_DATAA fd{};
  HANDLE find = FindFirstFileA(pattern.c_str(), &fd);
  if (find == INVALID_HANDLE_VALUE) return false;
  do {
    const std::string name = fd.cFileName;
    if (name == "." || name == "..") continue;
    const std::string full = JoinPath(dir, name);
    if (fd.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY)
      RemoveTreeRecursively(full);
    else
      DeleteFileA(full.c_str());
  } while (FindNextFileA(find, &fd));
  FindClose(find);
  return RemoveDirectoryA(dir.c_str()) != 0;
}

std::string JsonEscape(const std::string& s) {
  std::string out;
  for (char c : s) {
    switch (c) {
      case '"': out += "\\\""; break;
      case '\\': out += "\\\\"; break;
      case '\n': out += "\\n"; break;
      case '\r': out += "\\r"; break;
      case '\t': out += "\\t"; break;
      default:
        if (static_cast<unsigned char>(c) < 0x20) {
          char buf[8];
          std::snprintf(buf, sizeof(buf), "\\u%04x", c);
          out += buf;
        } else {
          out += c;
        }
    }
  }
  return out;
}

// ------------------------------------------------------------------ tracing

#define TRACE(...)                    \
  do {                                \
    std::printf("[trace] ");          \
    std::printf(__VA_ARGS__);         \
    std::printf("\n");                \
    std::fflush(stdout);              \
  } while (0)

void ProbeNotificationHandler(void*, RimeSessionId, const char* type,
                              const char* value) {
  TRACE("notify type=%s value=%s", type ? type : "(null)",
        value ? value : "(null)");
}

// ------------------------------------------------------------- rime wrapper

struct Rime {
  RimeApi* api = nullptr;

  bool Load(const std::string& dll_path, std::string* error) {
    HMODULE module = LoadLibraryA(dll_path.c_str());
    if (!module) {
      char msg[256];
      std::snprintf(msg, sizeof(msg),
                    "LoadLibraryA failed, GetLastError=%lu", GetLastError());
      *error = msg;
      return false;
    }
    // A x64 process can only load x64 DLLs: surviving LoadLibrary proves the
    // DLL is x64 (this probe is built x64).
    using GetApiFn = RimeApi* (*)(void);
    auto get_api = reinterpret_cast<GetApiFn>(GetProcAddress(
        module, "rime_get_api"));
    if (!get_api) {
      *error = "export rime_get_api not found (not a librime 1.x DLL)";
      return false;
    }
    api = get_api();
    if (!api) {
      *error = "rime_get_api returned NULL";
      return false;
    }
    if (!RIME_PROVIDED(api, setup) || !RIME_PROVIDED(api, initialize) ||
        !RIME_PROVIDED(api, start_maintenance) ||
        !RIME_PROVIDED(api, join_maintenance_thread) ||
        !RIME_PROVIDED(api, create_session) ||
        !RIME_PROVIDED(api, destroy_session) ||
        !RIME_PROVIDED(api, process_key) ||
        !RIME_PROVIDED(api, clear_composition) ||
        !RIME_PROVIDED(api, get_commit) || !RIME_PROVIDED(api, free_commit) ||
        !RIME_PROVIDED(api, get_context) ||
        !RIME_PROVIDED(api, free_context) ||
        !RIME_PROVIDED(api, get_status) || !RIME_PROVIDED(api, free_status) ||
        !RIME_PROVIDED(api, finalize)) {
      *error = "RimeApi struct lacks required members (librime too old)";
      return false;
    }
    return true;
  }

  bool TypeKey(RimeSessionId session, int keysym, const char* label) {
    const Bool consumed = api->process_key(session, keysym, 0);
    TRACE("process_key session=%zu key=%s(0x%x) -> consumed=%d",
          static_cast<size_t>(session), label, keysym, consumed);
    return consumed != 0;
  }

  bool TypeString(RimeSessionId session, const char* keys) {
    for (const char* p = keys; *p; ++p) {
      char label[2] = {*p, 0};
      if (!TypeKey(session, static_cast<int>(*p), label)) return false;
    }
    return true;
  }

  bool GetPreedit(RimeSessionId session, std::string* out) {
    RIME_STRUCT(RimeContext, ctx);
    if (!api->get_context(session, &ctx)) return false;
    if (ctx.composition.preedit) *out = ctx.composition.preedit;
    api->free_context(&ctx);
    return true;
  }

  bool GetFirstCandidate(RimeSessionId session, std::string* out) {
    RIME_STRUCT(RimeContext, ctx);
    if (!api->get_context(session, &ctx)) return false;
    if (ctx.menu.num_candidates > 0 && ctx.menu.candidates &&
        ctx.menu.candidates[0].text)
      *out = ctx.menu.candidates[0].text;
    api->free_context(&ctx);
    return true;
  }

  bool GetStatusSchema(RimeSessionId session, std::string* schema_id,
                       std::string* schema_name) {
    RIME_STRUCT(RimeStatus, status);
    if (!api->get_status(session, &status)) return false;
    if (status.schema_id) *schema_id = status.schema_id;
    if (status.schema_name) *schema_name = status.schema_name;
    api->free_status(&status);
    return true;
  }

  // Returns the pending commit text; *has_commit reports whether there was
  // an unread commit at all.
  std::string TakeCommit(RimeSessionId session, bool* has_commit) {
    RIME_STRUCT(RimeCommit, commit);
    std::string text;
    *has_commit = api->get_commit(session, &commit) != 0;
    if (*has_commit) {
      if (commit.text) text = commit.text;
      api->free_commit(&commit);
    }
    return text;
  }
};

struct TestResult {
  const char* name;
  bool pass;
  std::string detail;
};

int PrintUsageError(const char* msg) {
  std::printf("ERROR bad usage: %s\n", msg);
  std::printf("usage: rime_probe.exe [--rime-dll PATH] [--shared-data-dir DIR]"
              " [--user-data-dir DIR] [--schema SCHEMA_ID] [--keep]\n");
  return 2;
}

}  // namespace

int main(int argc, char** argv) {
  SetConsoleOutputCP(CP_UTF8);
#ifdef _DEBUG
  // Debug CRT: route assertion/error reports to the debugger and stderr.
  // The default modal MessageBox silently blocks a console tool forever.
  _CrtSetReportMode(_CRT_WARN, _CRTDBG_MODE_DEBUG | _CRTDBG_MODE_FILE);
  _CrtSetReportFile(_CRT_WARN, _CRTDBG_FILE_STDERR);
  _CrtSetReportMode(_CRT_ERROR, _CRTDBG_MODE_DEBUG | _CRTDBG_MODE_FILE);
  _CrtSetReportFile(_CRT_ERROR, _CRTDBG_FILE_STDERR);
  _CrtSetReportMode(_CRT_ASSERT, _CRTDBG_MODE_DEBUG | _CRTDBG_MODE_FILE);
  _CrtSetReportFile(_CRT_ASSERT, _CRTDBG_FILE_STDERR);
#endif
  // Unbuffered: every trace/result line reaches the log even if the engine
  // misbehaves later.
  std::setvbuf(stdout, nullptr, _IONBF, 0);
  Options opt;

  for (int i = 1; i < argc; ++i) {
    const std::string arg = argv[i];
    auto value = [&]() -> const char* {
      if (i + 1 >= argc) return nullptr;
      return argv[++i];
    };
    if (arg == "--rime-dll") {
      const char* v = value();
      if (!v) return PrintUsageError("--rime-dll needs a value");
      opt.rime_dll = v;
    } else if (arg == "--shared-data-dir") {
      const char* v = value();
      if (!v) return PrintUsageError("--shared-data-dir needs a value");
      opt.shared_data_dir = v;
    } else if (arg == "--user-data-dir") {
      const char* v = value();
      if (!v) return PrintUsageError("--user-data-dir needs a value");
      opt.user_data_dir = v;
      opt.user_dir_from_temp = false;
    } else if (arg == "--schema") {
      const char* v = value();
      if (!v) return PrintUsageError("--schema needs a value");
      opt.schema = v;
    } else if (arg == "--keep") {
      opt.keep_user_dir = true;
    } else {
      std::printf("unknown arg: %s\n", arg.c_str());
      return PrintUsageError(arg.c_str());
    }
  }

  // ------------------------------------------------- resolve dll + data dirs
  std::string weasel_root;
  const bool have_registry = ReadWeaselRootFromRegistry(&weasel_root);
  TRACE("registry WeaselRoot=%s",
        have_registry ? weasel_root.c_str() : "(not found)");

  if (opt.rime_dll.empty()) {
    if (have_registry) opt.rime_dll = JoinPath(weasel_root, "rime.dll");
    if (opt.rime_dll.empty() || !FileExists(opt.rime_dll)) {
      std::printf(
          "ERROR [missing-dll] rime.dll not found.\n"
          "  searched: registry WeaselRoot\\rime.dll (%s)\n"
          "  fix: install Weasel 0.17.4 x64 or pass --rime-dll PATH\n",
          opt.rime_dll.c_str());
      return 2;
    }
  }
  if (!FileExists(opt.rime_dll)) {
    std::printf("ERROR [missing-dll] --rime-dll path not found: %s\n",
                opt.rime_dll.c_str());
    return 2;
  }

  if (opt.shared_data_dir.empty()) {
    if (!have_registry) {
      std::printf(
          "ERROR [missing-dll] WeaselRoot not in registry and no "
          "--shared-data-dir given\n");
      return 2;
    }
    opt.shared_data_dir = JoinPath(weasel_root, "data");
  }
  const std::string schema_file =
      JoinPath(opt.shared_data_dir, opt.schema + ".schema.yaml");
  if (!FileExists(schema_file)) {
    std::printf(
        "ERROR [missing-schema] schema file not found: %s\n"
        "  fix: pass --shared-data-dir DIR containing %s.schema.yaml\n",
        schema_file.c_str(), opt.schema.c_str());
    return 3;
  }

  // ------------------------------------------------------ prepare user dir
  if (opt.user_data_dir.empty()) {
    char temp[MAX_PATH]{};
    GetTempPathA(MAX_PATH, temp);
    char uniq[64];
    std::snprintf(uniq, sizeof(uniq), "autoime-probe-%lu",
                  static_cast<unsigned long>(GetTickCount()));
    opt.user_data_dir = JoinPath(temp, uniq);
    if (FileExists(opt.user_data_dir)) {
      std::printf("ERROR user dir already exists (not empty): %s\n",
                  opt.user_data_dir.c_str());
      return 4;
    }
  }
  if (!CreateDirectoryA(opt.user_data_dir.c_str(), nullptr)) {
    const DWORD err = GetLastError();
    if (err != ERROR_ALREADY_EXISTS || !opt.user_dir_from_temp) {
      if (err != ERROR_ALREADY_EXISTS) {
        std::printf("ERROR cannot create user dir %s (GetLastError=%lu)\n",
                    opt.user_data_dir.c_str(), err);
        return 4;
      }
    }
    if (err == ERROR_ALREADY_EXISTS && opt.user_dir_from_temp) {
      std::printf("ERROR user dir already exists (not empty): %s\n",
                  opt.user_data_dir.c_str());
      return 4;
    }
  }
  TRACE("user_data_dir=%s%s", opt.user_data_dir.c_str(),
        opt.user_dir_from_temp ? " (fresh, empty)" : "");

  // Pin the schema list to the requested schema via a user-level patch.
  {
    const std::string custom = JoinPath(opt.user_data_dir,
                                        "default.custom.yaml");
    FILE* f = std::fopen(custom.c_str(), "wb");
    if (!f) {
      std::printf("ERROR cannot write %s\n", custom.c_str());
      return 4;
    }
    std::fprintf(f,
                 "# AutoIME probe: pin schema list (PLAN-069 T-01)\n"
                 "patch:\n"
                 "  schema_list:\n"
                 "    - schema: %s\n",
                 opt.schema.c_str());
    std::fclose(f);
  }

  // ------------------------------------------------------------ load engine
  Rime rime;
  std::string error;
  if (!rime.Load(opt.rime_dll, &error)) {
    std::printf("ERROR [missing-dll] cannot load %s: %s\n",
                opt.rime_dll.c_str(), error.c_str());
    if (!opt.keep_user_dir) RemoveTreeRecursively(opt.user_data_dir);
    return 2;
  }
  const char* rime_version_raw =
      RIME_PROVIDED(rime.api, get_version) ? rime.api->get_version()
                                           : "n/a";
  // Copy immediately: not safe to dereference the engine-owned pointer after
  // finalize().
  const std::string rime_version = rime_version_raw ? rime_version_raw : "n/a";
  TRACE("rime.dll loaded: path=%s file_version=%s librime_version=%s "
        "probe_bits=%d",
        opt.rime_dll.c_str(), DllFileVersion(opt.rime_dll).c_str(),
        rime_version.c_str(),
        static_cast<int>(sizeof(void*) * 8));

  // ------------------------------------------------------------ deploy
  RIME_STRUCT(RimeTraits, traits);
  traits.shared_data_dir = opt.shared_data_dir.c_str();
  traits.user_data_dir = opt.user_data_dir.c_str();
  traits.distribution_name = "AutoIME";
  traits.distribution_code_name = "autoime-probe";
  traits.distribution_version = "0.1.0";
  traits.app_name = "rime.autoime-probe";

  rime.api->set_notification_handler(ProbeNotificationHandler, nullptr);
  TRACE("setup(shared=%s user=%s)", opt.shared_data_dir.c_str(),
        opt.user_data_dir.c_str());
  rime.api->setup(&traits);
  TRACE("initialize");
  rime.api->initialize(&traits);
  TRACE("start_maintenance(full_check=True)");
  rime.api->start_maintenance(True);
  if (rime.api->is_maintenance_mode()) {
    TRACE("maintenance running, joining thread");
    rime.api->join_maintenance_thread();
  }
  TRACE("maintenance done");

  // ---------------------------------------------------- schema sanity check
  const RimeSessionId session = rime.api->create_session();
  TRACE("create_session -> %zu", static_cast<size_t>(session));
  if (session == 0) {
    std::printf("ERROR [deploy] create_session returned 0\n");
    if (!opt.keep_user_dir) RemoveTreeRecursively(opt.user_data_dir);
    rime.api->finalize();
    return 4;
  }
  std::string schema_id, schema_name;
  if (!rime.GetStatusSchema(session, &schema_id, &schema_name) ||
      schema_id != opt.schema) {
    std::printf(
        "ERROR [deploy] current schema is '%s' (name '%s'), expected '%s'; "
        "deployment of %s likely failed (see notify lines above)\n",
        schema_id.c_str(), schema_name.c_str(), opt.schema.c_str(),
        schema_file.c_str());
    rime.api->destroy_session(session);
    if (!opt.keep_user_dir) RemoveTreeRecursively(opt.user_data_dir);
    rime.api->finalize();
    return 4;
  }
  TRACE("schema current: id=%s name=%s", schema_id.c_str(),
        schema_name.c_str());

  // ---------------------------------------------------------------- tests
  std::vector<TestResult> results;

  // t1: nihao -> "ni hao" -> select #1 -> exactly one commit "你好"
  {
    TestResult r{"t1_first_commit", false, ""};
    std::string preedit;
    bool ok = rime.TypeString(session, "nihao");
    ok = ok && rime.GetPreedit(session, &preedit);
    TRACE("t1 preedit after nihao: '%s'", preedit.c_str());
    const std::string expected_preedit = "ni hao";
    if (!ok) {
      r.detail = "process_key or get_context failed";
    } else if (preedit != expected_preedit) {
      r.detail = "preedit='" + preedit + "' expected='" + expected_preedit +
                 "'";
    } else {
      std::string first;
      rime.GetFirstCandidate(session, &first);
      TRACE("t1 first candidate: '%s'", first.c_str());
      ok = rime.TypeKey(session, '1', "1");  // select candidate #1
      bool has = false;
      const std::string committed = rime.TakeCommit(session, &has);
      TRACE("t1 commit: has=%d text='%s'", has, committed.c_str());
      std::string preedit_after;
      rime.GetPreedit(session, &preedit_after);
      bool has2 = false;
      rime.TakeCommit(session, &has2);
      TRACE("t1 second get_commit: has=%d (must be 0)", has2);
      if (!ok) {
        r.detail = "select key '1' not consumed";
      } else if (!has) {
        r.detail = "no commit after selecting candidate #1";
      } else if (committed != "\xE4\xBD\xA0\xE5\xA5\xBD") {
        r.detail = "commit text='" + committed + "' expected='你好'";
      } else if (!preedit_after.empty()) {
        r.detail = "composition not empty after commit: '" + preedit_after +
                   "'";
      } else if (has2) {
        r.detail = "more than one commit produced";
      } else {
        r.pass = true;
      }
    }
    results.push_back(r);
  }

  // t2: "ni" + Esc cancels, no commit
  {
    rime.api->clear_composition(session);
    TestResult r{"t2_cancel", false, ""};
    bool ok = rime.TypeString(session, "ni");
    std::string preedit;
    ok = ok && rime.GetPreedit(session, &preedit);
    const bool esc_consumed = rime.TypeKey(session, kKeysymEscape, "Esc");
    std::string preedit_after;
    rime.GetPreedit(session, &preedit_after);
    bool has = false;
    rime.TakeCommit(session, &has);
    TRACE("t2 preedit='%s' esc_consumed=%d after='%s' commit=%d",
          preedit.c_str(), esc_consumed, preedit_after.c_str(), has);
    if (!ok || preedit.empty()) {
      r.detail = "could not compose 'ni'";
    } else if (!esc_consumed) {
      r.detail = "Esc not consumed";
    } else if (!preedit_after.empty()) {
      r.detail = "composition still '" + preedit_after + "' after Esc";
    } else if (has) {
      r.detail = "commit produced after Esc (must be none)";
    } else {
      r.pass = true;
    }
    results.push_back(r);
  }

  // t3: two sessions isolated (composition + candidates)
  {
    TestResult r{"t3_two_sessions", false, ""};
    const RimeSessionId s2 = rime.api->create_session();
    TRACE("create_session(2nd) -> %zu", static_cast<size_t>(s2));
    bool ok = s2 != 0 && s2 != session;
    // fresh state for the first session
    rime.api->clear_composition(session);
    ok = ok && rime.TypeString(session, "ni");
    ok = ok && rime.TypeString(s2, "hao");
    std::string pre1, pre2, cand1, cand2;
    ok = ok && rime.GetPreedit(session, &pre1);
    ok = ok && rime.GetPreedit(s2, &pre2);
    ok = ok && rime.GetFirstCandidate(session, &cand1);
    ok = ok && rime.GetFirstCandidate(s2, &cand2);
    bool has1 = false, has2 = false;
    rime.TakeCommit(session, &has1);
    rime.TakeCommit(s2, &has2);
    TRACE("t3 A: preedit='%s' cand0='%s' B: preedit='%s' cand0='%s' "
          "commits=%d/%d",
          pre1.c_str(), cand1.c_str(), pre2.c_str(), cand2.c_str(), has1,
          has2);
    if (!ok) {
      r.detail = "engine calls failed";
    } else if (pre1 != "ni") {
      r.detail = "A preedit='" + pre1 + "' expected 'ni'";
    } else if (pre2 != "hao") {
      r.detail = "B preedit='" + pre2 + "' expected 'hao'";
    } else if (cand1.empty() || cand2.empty()) {
      r.detail = "missing candidates";
    } else if (cand1 == cand2) {
      r.detail = "candidate lists not isolated: both '" + cand1 + "'";
    } else if (has1 || has2) {
      r.detail = "unexpected commits";
    } else {
      r.pass = true;
    }
    results.push_back(r);
    rime.api->destroy_session(s2);
  }

  // ---------------------------------------------------------------- summary
  rime.api->destroy_session(session);
  rime.api->finalize();
  TRACE("finalize");

  if (!opt.keep_user_dir && opt.user_dir_from_temp)
    RemoveTreeRecursively(opt.user_data_dir);
  TRACE("cleanup done (user_dir_removed=%d)",
        !opt.keep_user_dir && opt.user_dir_from_temp);

  bool all_pass = true;
  for (const TestResult& r : results) {
    std::printf("RESULT %s=%s%s%s\n", r.name, r.pass ? "PASS" : "FAIL",
                r.detail.empty() ? "" : " detail: ", r.detail.c_str());
    all_pass = all_pass && r.pass;
  }
  std::printf(
      "SUMMARY {\"probe\": \"rime_probe\", \"dll\": \"%s\", "
      "\"dll_file_version\": \"%s\", \"librime_version\": \"%s\", "
      "\"probe_bits\": %d, \"schema\": \"%s\", \"schema_name\": \"%s\", "
      "\"shared_data_dir\": \"%s\", \"user_data_dir\": \"%s\", "
      "\"tests\": {\"t1_first_commit\": \"%s\", \"t2_cancel\": \"%s\", "
      "\"t3_two_sessions\": \"%s\"}}\n",
      JsonEscape(opt.rime_dll).c_str(),
      JsonEscape(DllFileVersion(opt.rime_dll)).c_str(),
      JsonEscape(rime_version).c_str(),
      static_cast<int>(sizeof(void*) * 8), JsonEscape(schema_id).c_str(),
      JsonEscape(schema_name).c_str(),
      JsonEscape(opt.shared_data_dir).c_str(),
      JsonEscape(opt.user_data_dir).c_str(),
      results[0].pass ? "pass" : "fail", results[1].pass ? "pass" : "fail",
      results[2].pass ? "pass" : "fail");
  // Terminate without running atexit handlers / DLL unload: rime.dll keeps
  // background threads and static state that make the CRT exit path unreliable
  // after finalize(); all engine state is fully torn down above.
  TRACE("exiting via _exit (skip CRT exit: engine static teardown unreliable)");
  _exit(all_pass ? 0 : 5);
}
