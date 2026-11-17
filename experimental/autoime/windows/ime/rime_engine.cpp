// AutoIME - narrow librime adapter implementation (PLAN-069 T-03).
#include "rime_engine.h"

#include <cstdio>
#include <cstring>

#include "ime.h"

namespace {

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

std::string ReadRegistryWeaselRoot() {
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
  return st == ERROR_SUCCESS ? std::string(buf) : std::string();
}

std::wstring Utf8ToUtf16(const std::string& utf8) {
  if (utf8.empty()) return std::wstring();
  const int n = MultiByteToWideChar(CP_UTF8, 0, utf8.c_str(),
                                    static_cast<int>(utf8.size()), nullptr, 0);
  std::wstring out(n, L'\0');
  MultiByteToWideChar(CP_UTF8, 0, utf8.c_str(), static_cast<int>(utf8.size()),
                      &out[0], n);
  return out;
}

}  // namespace

std::string RimeEngine::EnvOr(const char* env, const std::string& fallback) {
  char buf[MAX_PATH]{};
  const DWORD n = GetEnvironmentVariableA(env, buf, MAX_PATH);
  if (n > 0 && n < MAX_PATH) return buf;
  return fallback;
}

bool RimeEngine::LoadDll(std::string* error) {
  // Resolution order: env override -> registry WeaselRoot -> error.
  std::string weasel_root = ReadRegistryWeaselRoot();
  dll_path_ = EnvOr("AUTOIME_RIME_DLL",
                    weasel_root.empty() ? std::string()
                                        : JoinPath(weasel_root, "rime.dll"));
  shared_data_dir_ = EnvOr("AUTOIME_RIME_SHARED_DATA_DIR",
                           weasel_root.empty() ? std::string()
                                               : JoinPath(weasel_root, "data"));
  if (!dll_path_.empty() && !FileExists(dll_path_)) {
    *error = "rime.dll not found: " + dll_path_;
    return false;
  }
  if (dll_path_.empty() || shared_data_dir_.empty()) {
    *error = "Weasel not found (registry WeaselRoot missing); set "
             "AUTOIME_RIME_DLL and AUTOIME_RIME_SHARED_DATA_DIR";
    return false;
  }
  module_ = LoadLibraryA(dll_path_.c_str());
  if (!module_) {
    char msg[128];
    snprintf(msg, sizeof(msg), "LoadLibrary failed (GetLastError=%lu)",
             GetLastError());
    *error = std::string(msg) + ": " + dll_path_;
    return false;
  }
  using GetApiFn = RimeApi* (*)(void);
  auto get_api = reinterpret_cast<GetApiFn>(GetProcAddress(module_,
                                                           "rime_get_api"));
  if (!get_api || !(api_ = get_api()) || !RIME_PROVIDED(api_, setup) ||
      !RIME_PROVIDED(api_, initialize) ||
      !RIME_PROVIDED(api_, start_maintenance) ||
      !RIME_PROVIDED(api_, join_maintenance_thread) ||
      !RIME_PROVIDED(api_, create_session) ||
      !RIME_PROVIDED(api_, destroy_session) ||
      !RIME_PROVIDED(api_, process_key) ||
      !RIME_PROVIDED(api_, get_commit) || !RIME_PROVIDED(api_, free_commit) ||
      !RIME_PROVIDED(api_, get_context) ||
      !RIME_PROVIDED(api_, free_context) ||
      !RIME_PROVIDED(api_, clear_composition) ||
      !RIME_PROVIDED(api_, select_candidate)) {
    *error = "rime_get_api unavailable or RimeApi too old";
    return false;
  }
  return true;
}

void RimeEngine::WriteDefaultCustomYaml() const {
  const std::string custom = JoinPath(user_data_dir_, "default.custom.yaml");
  if (FileExists(custom)) return;
  FILE* f = fopen(custom.c_str(), "wb");
  if (!f) return;
  fprintf(f,
          "# AutoIME (PLAN-069): pin schema list\n"
          "patch:\n"
          "  schema_list:\n"
          "    - schema: %s\n",
          schema_.c_str());
  fclose(f);
}

void RimeEngine::InitializeOnce() {
  if (!ready_) return;
  // AutoIME's own user data dir (not the system Weasel user dir).
  user_data_dir_ =
      EnvOr("AUTOIME_RIME_USER_DATA_DIR",
            JoinPath(EnvOr("LOCALAPPDATA", ""), "AutoIME\\rime"));
  CreateDirectoryA(user_data_dir_.c_str(), nullptr);
  schema_ = EnvOr("AUTOIME_RIME_SCHEMA", "luna_pinyin_simp");
  WriteDefaultCustomYaml();

  RIME_STRUCT(RimeTraits, traits);
  traits.shared_data_dir = shared_data_dir_.c_str();
  traits.user_data_dir = user_data_dir_.c_str();
  traits.distribution_name = "AutoIME";
  traits.distribution_code_name = "autoime-ime";
  traits.distribution_version = "0.1.0";
  traits.app_name = "rime.autoime";
  api_->set_notification_handler(nullptr, nullptr);
  api_->setup(&traits);
  api_->initialize(&traits);
  api_->start_maintenance(False);
  if (api_->is_maintenance_mode()) api_->join_maintenance_thread();
  ImeLog("rime ready dll=%s user=%s schema=%s", dll_path_.c_str(),
         user_data_dir_.c_str(), schema_.c_str());
}

bool RimeEngine::EnsureReady(std::string* error) {
  if (ready_) return true;
  if (!LoadDll(error)) return false;
  ready_ = true;
  InitializeOnce();
  return true;
}

uintptr_t RimeEngine::CreateSession() {
  return api_ ? api_->create_session() : 0;
}

void RimeEngine::DestroySession(uintptr_t session) {
  if (api_ && session) api_->destroy_session(session);
}

bool RimeEngine::ProcessKey(uintptr_t session, int keysym, int modifiers) {
  return api_ && session && api_->process_key(session, keysym, modifiers);
}

bool RimeEngine::GetStatus(uintptr_t session, bool* ascii_mode, bool* disabled) {
  if (!api_ || !RIME_PROVIDED(api_, get_status)) return false;
  RIME_STRUCT(RimeStatus, status);
  if (!session || !api_->get_status(session, &status)) return false;
  if (ascii_mode) *ascii_mode = status.is_ascii_mode != 0;
  if (disabled) *disabled = status.is_disabled != 0;
  api_->free_status(&status);
  return true;
}

void RimeEngine::SetOption(uintptr_t session, const char* option, bool value) {
  if (api_ && session && option && RIME_PROVIDED(api_, set_option))
    api_->set_option(session, option, value ? True : False);
}

bool RimeEngine::GetContext(uintptr_t session, Snapshot* out) {
  if (!api_ || !session) return false;
  RIME_STRUCT(RimeContext, ctx);
  if (!api_->get_context(session, &ctx)) return false;
  if (ctx.composition.preedit)
    out->preedit_utf8 = ctx.composition.preedit;
  out->is_composing = ctx.composition.length > 0;
  for (int i = 0; i < ctx.menu.num_candidates && ctx.menu.candidates; ++i) {
    if (ctx.menu.candidates[i].text)
      out->candidates.push_back(Utf8ToUtf16(ctx.menu.candidates[i].text));
  }
  out->highlighted = ctx.menu.highlighted_candidate_index;
  out->page_no = ctx.menu.page_no;
  out->is_last_page = ctx.menu.is_last_page != 0;
  if (ctx.commit_text_preview)
    out->commit_preview_utf8 = ctx.commit_text_preview;
  api_->free_context(&ctx);
  return true;
}

std::string RimeEngine::TakeCommit(uintptr_t session, bool* has_commit) {
  *has_commit = false;
  if (!api_ || !session) return "";
  RIME_STRUCT(RimeCommit, commit);
  std::string text;
  if (api_->get_commit(session, &commit)) {
    *has_commit = true;
    if (commit.text) text = commit.text;
    api_->free_commit(&commit);
  }
  return text;
}

void RimeEngine::ClearComposition(uintptr_t session) {
  if (api_ && session) api_->clear_composition(session);
}

void RimeEngine::SelectCandidate(uintptr_t session, size_t index) {
  if (api_ && session && RIME_PROVIDED(api_, select_candidate))
    api_->select_candidate(session, index);
}
