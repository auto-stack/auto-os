// AutoIME Windows x64 - TSF text service (PLAN-069 T-02/T-03, PLAN-070 T-01).
// Minimal x64 TSF keyboard TIP: registers with its own CLSID and language
// profile, routes keys through a deduplicating router, and applies the Rime
// composition/commit result to the host's edit context.
#ifndef AUTOIME_IME_H_
#define AUTOIME_IME_H_

#include <windows.h>
#include <msctf.h>

#include "candidate_window.h"
#include "key_router.h"
#include "rime_engine.h"

#include <cstdio>
#include <string>

// {02e1e367-4389-429e-b13c-483eb5555d37}
DEFINE_GUID(CLSID_AutoIMETextService,
            0x02e1e367, 0x4389, 0x429e, 0xb1, 0x3c, 0x48, 0x3e, 0xb5, 0x55,
            0x5d, 0x37);
// AutoIME language profile (zh-CN 0x0804) {e8713320-7ae4-40d8-9850-af2ea17afd00}
DEFINE_GUID(GUID_PROFILE_AUTOIME,
            0xe8713320, 0x7ae4, 0x40d8, 0x98, 0x50, 0xaf, 0x2e, 0xa1, 0x7a,
            0xfd, 0x00);

constexpr LANGID kAutoIMElangid = 0x0804;  // Chinese (Simplified)

// Append a structural line to %TEMP%\autoime-ime.log (best effort, never
// fatal). This entry point is for identity/paths/rectangles/HRESULTs only;
// key codes, pre-edit, selection and committed text must never be passed here.
void ImeLog(const char* fmt, ...);

// COM object count shared by DllCanUnloadNow (dllmain.cpp).
void AutoIMEAddRefObject();
void AutoIMEReleaseObject();
void AutoIMELockServer(BOOL fLock);

extern HINSTANCE g_hInst;

class CClassFactory final : public IClassFactory {
 public:
  CClassFactory() : ref_(1) { AutoIMEAddRefObject(); }

  STDMETHODIMP QueryInterface(REFIID riid, void** ppv) override;
  STDMETHODIMP_(ULONG) AddRef() override { return InterlockedIncrement(&ref_); }
  STDMETHODIMP_(ULONG) Release() override {
    const ULONG cr = InterlockedDecrement(&ref_);
    if (cr == 0) {
      AutoIMEReleaseObject();
      delete this;
    }
    return cr;
  }
  STDMETHODIMP CreateInstance(IUnknown* pUnkOuter, REFIID riid,
                              void** ppvObject) override;
  STDMETHODIMP LockServer(BOOL fLock) override;

 private:
  LONG ref_ = 1;
};

class CTextService final : public ITfTextInputProcessor,
                           public ITfKeyEventSink,
                           public ITfCompositionSink,
                           private IKeyTarget {
 public:
  CTextService() : ref_(1) { AutoIMEAddRefObject(); }

  // IUnknown
  STDMETHODIMP QueryInterface(REFIID riid, void** ppv) override;
  STDMETHODIMP_(ULONG) AddRef() override { return InterlockedIncrement(&ref_); }
  STDMETHODIMP_(ULONG) Release() override {
    const ULONG cr = InterlockedDecrement(&ref_);
    if (cr == 0) {
      AutoIMEReleaseObject();
      delete this;
    }
    return cr;
  }

  // ITfTextInputProcessor
  STDMETHODIMP Activate(ITfThreadMgr* ptim, TfClientId tid) override;
  STDMETHODIMP Deactivate() override;

  // ITfKeyEventSink
  STDMETHODIMP OnSetFocus(BOOL fForeground) override;
  STDMETHODIMP OnTestKeyDown(ITfContext* pic, WPARAM wParam, LPARAM lParam,
                             BOOL* pfEaten) override;
  STDMETHODIMP OnTestKeyUp(ITfContext* pic, WPARAM wParam, LPARAM lParam,
                           BOOL* pfEaten) override;
  STDMETHODIMP OnKeyDown(ITfContext* pic, WPARAM wParam, LPARAM lParam,
                         BOOL* pfEaten) override;
  STDMETHODIMP OnKeyUp(ITfContext* pic, WPARAM wParam, LPARAM lParam,
                       BOOL* pfEaten) override;
  STDMETHODIMP OnPreservedKey(ITfContext* pic, REFGUID rguid,
                              BOOL* pfEaten) override;

  // ITfCompositionSink
  STDMETHODIMP OnCompositionTerminated(TfEditCookie ecWrite,
                                       ITfComposition* pComposition) override;

 private:
  // IKeyTarget: the router owns the callback de-duplication policy; these
  // hooks expose the engine/composition state and apply one key exactly once.
  bool Composing() override;
  bool AsciiMode() override;
  bool Sensitive() override;
  bool EngineReady() override;
  bool ProcessKey(int keysym, int rime_modifiers) override;
  void ToggleMode() override;
  void CancelComposition() override;

  bool AdviseKeyEventSink();
  void UnadviseKeyEventSink();
  bool EnsureEngine();
  bool IsSensitiveContext(ITfContext* pic);
  void ApplyCompositionState(ITfContext* pic, bool has_commit,
                             const std::string& commit_utf8,
                             const RimeEngine::Snapshot& snap);
  bool EnsureComposition(ITfContext* pic);
  HRESULT UpdateCompositionText(ITfContext* pic, const std::wstring& text);
  HRESULT FinalizeComposition(ITfContext* pic, const std::wstring& text);
  void DiscardComposition(ITfContext* pic);
  void RefreshCandidateWindow(ITfContext* pic);
  KeyEvent MakeKeyEvent(WPARAM wParam, LPARAM lParam) const;

  LONG ref_ = 1;
  ITfThreadMgr* thread_mgr_ = nullptr;
  TfClientId client_id_ = 0;
  bool key_sink_advised_ = false;

  RimeEngine engine_;
  uintptr_t rime_session_ = 0;
  ITfContext* active_ctx_ = nullptr;   // context owning the open composition
  ITfContext* current_ctx_ = nullptr;  // context of the callback in flight
  ITfComposition* composition_ = nullptr;
  std::wstring composition_text_;
  CandidateWindow candidate_wnd_;

  KeyRouter router_;
  uint64_t generation_ = 1;
  bool engine_composing_ = false;
  bool ascii_mode_ = false;
  bool engine_load_failed_ = false;
};

// Registry self-registration (register.cpp). Install failures roll back the
// keys added during this call; unregister only ever touches AutoIME keys.
HRESULT AutoIMESelfRegister(HINSTANCE instance);
HRESULT AutoIMESelfUnregister();

#endif  // AUTOIME_IME_H_
