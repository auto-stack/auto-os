// AutoIME - narrow librime adapter implementation (PLAN-069 T-03).
#include "rime_engine.h"

#include <atomic>
#include <cstdio>
#include <cstring>

#include "deployment.h"
#include "diag.h"
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

uint64_t NextSnapshotVersion() {
  static std::atomic<uint64_t> counter{0};
  return ++counter;
}

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

// Pins AutoIME's schema list in the private user dir. A file the user owns
// (no AutoIME marker) is left untouched.
void RimeEngine::WriteDefaultCustomYaml() const {
  const std::string custom = JoinPath(user_data_dir_, "default.custom.yaml");
  std::string existing;
  {
    FILE* f = std::fopen(custom.c_str(), "rb");
    if (f) {
      char buffer[4096];
      const size_t n = std::fread(buffer, 1, sizeof(buffer), f);
      existing.assign(buffer, n);
      std::fclose(f);
    }
  }
  if (!existing.empty() && existing.find("# AutoIME") != 0) return;
  FILE* f = std::fopen(custom.c_str(), "wb");
  if (!f) return;
  std::fprintf(f,
               "# AutoIME (PLAN-070): pinned schema list with Luna fallback\n"
               "patch:\n"
               "  schema_list:\n"
               "    - schema: %s\n",
               schema_.c_str());
  if (schema_ != "luna_pinyin_simp")
    std::fprintf(f, "    - schema: luna_pinyin_simp\n");
  std::fclose(f);
}

bool RimeEngine::InitializeAndMaintain() {
  if (!api_) return false;
  CreateDirectoryA(user_data_dir_.c_str(), nullptr);
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
  api_->start_maintenance(True);
  if (api_->is_maintenance_mode()) api_->join_maintenance_thread();
  return true;
}

void RimeEngine::FinalizeEngine() {
  if (api_) api_->finalize();
}

bool RimeEngine::ValidateSchema(const std::string& expected,
                                std::string* actual) {
  if (!api_ || !RIME_PROVIDED(api_, get_status)) return false;
  const RimeSessionId session = api_->create_session();
  if (!session) return false;
  // Validate the named schema itself, independent of the user's previous
  // explicit selection (which is preserved for the runtime sessions).
  if (RIME_PROVIDED(api_, select_schema))
    api_->select_schema(session, expected.c_str());
  std::string id, name;
  RIME_STRUCT(RimeStatus, status);
  if (api_->get_status(session, &status)) {
    if (status.schema_id) id = status.schema_id;
    if (status.schema_name) name = status.schema_name;
    api_->free_status(&status);
  }
  // A schema whose YAML failed to parse still reports its declared id (with
  // the display name falling back to the id), and a stale compiled artifact
  // can keep working. Require both an effective display name and a functional
  // composition probe before declaring the deployed set usable.
  bool functional = false;
  const char* probe = "nihao";
  for (const char* p = probe; *p; ++p) api_->process_key(session, *p, 0);
  RIME_STRUCT(RimeContext, ctx);
  if (api_->get_context(session, &ctx)) {
    functional = ctx.menu.num_candidates > 0 && ctx.menu.candidates &&
                 ctx.menu.candidates[0].text &&
                 ctx.menu.candidates[0].text[0] != '\0';
    api_->free_context(&ctx);
  }
  api_->clear_composition(session);
  api_->destroy_session(session);
  if (actual) *actual = id;
  ImeLog("schema validation id=%s name=%s expected=%s functional=%d",
         id.c_str(), name.c_str(), expected.c_str(), functional ? 1 : 0);
  return id == expected && !name.empty() && name != id && functional;
}

void RimeEngine::CommitResources() {
  if (!resources_dir_.empty()) deployment::Commit(user_data_dir_);
}

void RimeEngine::RollbackResources() {
  if (resources_dir_.empty()) return;
  std::string detail;
  deployment::Rollback(user_data_dir_, &detail);
  deployment_detail_ += " | " + detail;
}

RimeEngine::~RimeEngine() {
  if (worker_.joinable()) worker_.join();
}

void RimeEngine::StartAsync() {
  bool expected = false;
  if (!started_.compare_exchange_strong(expected, true)) return;
  worker_ = std::thread([this] {
    std::string error;
    if (!EnsureReady(&error))
      ImeLog("async engine initialization failed: %s", error.c_str());
  });
}

void RimeEngine::Shutdown() {
  if (worker_.joinable()) worker_.join();
  std::lock_guard<std::mutex> guard(init_mutex_);
  FinalizeEngine();
  ready_.store(false);
}

bool RimeEngine::EnsureReady(std::string* error) {
  std::lock_guard<std::mutex> guard(init_mutex_);
  if (finished_.load()) return ready_.load();
  if (!LoadDll(error)) {
    finished_.store(true);
    diag::Failure("engine", E_FAIL);
    return false;
  }
  user_data_dir_ =
      user_override_.empty() ? deployment::DefaultUserDir() : user_override_;
  CreateDirectoryA(user_data_dir_.c_str(), nullptr);
  resources_dir_ = resources_override_;
  if (resources_dir_.empty() && !module_dir_.empty())
    resources_dir_ = deployment::ResolveResourcesDir(module_dir_);

  // Hold the cross-process deployment gate across Prepare + compile + commit:
  // two hosts cold-starting together must compile the dictionaries only once.
  deployment::DeployLock deploy_lock;
  bool resources_ok = false;
  if (!resources_dir_.empty()) {
    const deployment::Status st =
        deployment::Prepare(resources_dir_, user_data_dir_);
    resources_ok = st.ok;
    deployment_version_ = st.version;
    deployment_detail_ = st.detail;
    ImeLog("deploy resources=%s version=%s copied=%d unchanged=%d detail=%s",
           resources_dir_.c_str(), st.version.c_str(), st.copied ? 1 : 0,
           st.unchanged ? 1 : 0, st.detail.c_str());
    diag::Event("deploy", st.ok ? (st.unchanged ? "unchanged" : "prepared")
                                : "failed");
  } else {
    deployment_detail_ = "no resources directory (Luna fallback)";
    diag::Event("deploy", "no_resources");
  }

  schema_ = resources_ok ? "autoime_pinyin" : "luna_pinyin_simp";
  const std::string env_schema = EnvOr("AUTOIME_RIME_SCHEMA", "");
  if (!env_schema.empty()) schema_ = env_schema;

  if (!InitializeAndMaintain()) {
    finished_.store(true);
    deployment_detail_ += " | initialize failed";
    return false;
  }
  std::string actual;
  if (ValidateSchema(schema_, &actual)) {
    if (resources_ok) CommitResources();
    ready_.store(true);
    diag::Event("engine", "ready");
  } else {
    // 1. Restore the last known-good resource set and retry once. Finalize
    // first so the compile output is no longer mapped and can be dropped.
    FinalizeEngine();
    RollbackResources();
    if (InitializeAndMaintain() && ValidateSchema(schema_, &actual)) {
      ready_.store(true);
      diag::Event("engine", "ready_after_rollback");
    } else {
      // 2. Last resort: Luna schema shipped with Weasel (no AutoIME data).
      schema_ = "luna_pinyin_simp";
      schema_override_ = "luna_pinyin_simp";
      fallback_.store(true);
      FinalizeEngine();
      if (InitializeAndMaintain() && ValidateSchema(schema_, &actual)) {
        ready_.store(true);
        diag::Event("engine", "ready_luna");
      } else {
        disabled_.store(true);
        ready_.store(false);
        FinalizeEngine();
        diag::Event("engine", "disabled");
        if (error) *error = "engine disabled: no usable schema";
      }
    }
  }
  finished_.store(true);
  return ready_.load();
}

uintptr_t RimeEngine::CreateSession() {
  if (!api_) return 0;
  const uintptr_t session = api_->create_session();
  if (session && !schema_override_.empty() &&
      RIME_PROVIDED(api_, select_schema))
    api_->select_schema(session, schema_override_.c_str());
  return session;
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
  *out = Snapshot{};
  if (ctx.composition.preedit) out->preedit_utf8 = ctx.composition.preedit;
  out->is_composing = ctx.composition.length > 0;
  out->page_no = ctx.menu.page_no;
  out->is_last_page = ctx.menu.is_last_page != 0;
  if (ctx.menu.select_keys) out->select_keys = ctx.menu.select_keys;
  // The engine reports the whole candidate list; the snapshot keeps only the
  // current page and pairs every entry with its real selection label
  // (PLAN-070 T-03: labels, page index and page version).
  const int total = ctx.menu.num_candidates;
  const int page_size =
      ctx.menu.page_size > 0 ? ctx.menu.page_size : (total > 0 ? total : 1);
  int begin = ctx.menu.page_no * page_size;
  if (begin >= total) begin = 0;
  int end = begin + page_size;
  if (end > total) end = total;
  for (int i = begin; i < end && ctx.menu.candidates; ++i) {
    out->candidates.push_back(ctx.menu.candidates[i].text
                                  ? Utf8ToUtf16(ctx.menu.candidates[i].text)
                                  : std::wstring());
    out->comments.push_back(ctx.menu.candidates[i].comment
                                ? Utf8ToUtf16(ctx.menu.candidates[i].comment)
                                : std::wstring());
    std::wstring label;
    if (ctx.select_labels && ctx.select_labels[i] &&
        ctx.select_labels[i][0]) {
      label = Utf8ToUtf16(ctx.select_labels[i]);
    } else {
      const int rel = i - begin;
      if (rel < static_cast<int>(out->select_keys.size())) {
        label = std::wstring(
            1, static_cast<wchar_t>(
                   static_cast<unsigned char>(out->select_keys[rel])));
      } else {
        label = std::to_wstring(rel + 1);
      }
    }
    out->labels.push_back(label);
  }
  out->highlighted = ctx.menu.highlighted_candidate_index;
  if (out->highlighted >= begin && out->highlighted < end)
    out->highlighted -= begin;
  else if (out->highlighted >= page_size)
    out->highlighted -= begin;
  out->version = NextSnapshotVersion();
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

bool RimeEngine::SelectCandidateOnPage(uintptr_t session, size_t page_index) {
  if (!api_ || !session) return false;
  if (RIME_PROVIDED(api_, select_candidate_on_current_page))
    return api_->select_candidate_on_current_page(session, page_index) != 0;
  return false;
}

void RimeEngine::SelectCandidate(uintptr_t session, size_t index) {
  if (api_ && session && RIME_PROVIDED(api_, select_candidate))
    api_->select_candidate(session, index);
}
