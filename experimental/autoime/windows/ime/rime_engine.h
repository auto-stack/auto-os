// AutoIME - narrow librime adapter (PLAN-069 T-03).
// Wraps session create / key press / context snapshot / commit / destroy.
// The DLL and data dirs resolve in order: env override (AUTOIME_RIME_DLL,
// AUTOIME_RIME_SHARED_DATA_DIR, AUTOIME_RIME_USER_DATA_DIR), then the Weasel
// registry key, then defaults. AutoIME writes only its own user data dir.
#ifndef AUTOIME_RIME_ENGINE_H_
#define AUTOIME_RIME_ENGINE_H_

#include <string>
#include <vector>

#include <windows.h>

#include "../probe/rime_api.h"

class RimeEngine {
 public:
  struct Snapshot {
    std::string preedit_utf8;       // composition preedit
    bool is_composing = false;
    std::vector<std::wstring> candidates;  // page 0..n, UTF-16 for GDI
    int highlighted = 0;
    int page_no = 0;
    bool is_last_page = true;
    std::string commit_preview_utf8;
  };

  // Loads rime.dll, deploys and initializes once per process. Safe to call
  // repeatedly; returns false with a diagnosable message on failure.
  bool EnsureReady(std::string* error);
  bool ready() const { return ready_; }
  const std::string& dll_path() const { return dll_path_; }
  const std::string& user_data_dir() const { return user_data_dir_; }
  const std::string& schema() const { return schema_; }

  uintptr_t CreateSession();
  void DestroySession(uintptr_t session);

  // Sends one key; returns true when Rime consumed it.
  bool ProcessKey(uintptr_t session, int keysym);
  // Reads the composition context; false when the engine call fails.
  bool GetContext(uintptr_t session, Snapshot* out);
  // Returns the pending commit text ("" when none) and consumes it.
  std::string TakeCommit(uintptr_t session, bool* has_commit);
  void ClearComposition(uintptr_t session);
  void SelectCandidate(uintptr_t session, size_t index);

 private:
  bool LoadDll(std::string* error);
  void InitializeOnce();
  void WriteDefaultCustomYaml() const;
  static std::string EnvOr(const char* env, const std::string& fallback);

  bool ready_ = false;
  RimeApi* api_ = nullptr;
  HMODULE module_ = nullptr;
  std::string dll_path_;
  std::string shared_data_dir_;
  std::string user_data_dir_;
  std::string schema_ = "luna_pinyin_simp";
};

// Maps a VK wParam (+ current modifiers are ignored for the preview) to an
// X11-style Rime keysym; returns 0 when the key is not routed to Rime.
int AutoIMEVkToKeysym(WPARAM vk, LPARAM lParam);

#endif  // AUTOIME_RIME_ENGINE_H_
