// AutoIME TSF text service - key routing, composition, candidates (T-03).
// Key flow (PLAN-069): OnTestKeyDown decides whether Rime wants the key by
// processing it and applying the resulting state; OnKeyDown returns the
// cached decision (and self-heals if a host skips Test). Unconsumed keys are
// returned to the host untouched. Failures in the engine layer degrade to
// pass-through so the user keeps a usable editor.
#include "ime.h"

#include <functional>
#include <new>

namespace {

std::wstring Utf8ToUtf16(const std::string& utf8) {
  if (utf8.empty()) return std::wstring();
  const int n = MultiByteToWideChar(CP_UTF8, 0, utf8.c_str(),
                                    static_cast<int>(utf8.size()), nullptr, 0);
  std::wstring out(n, L'\0');
  MultiByteToWideChar(CP_UTF8, 0, utf8.c_str(), static_cast<int>(utf8.size()),
                      &out[0], n);
  return out;
}

// Synchronous edit session runner: must be called on the UI thread while
// handling key events (TF_ES_SYNC).
class SyncEditSession final : public ITfEditSession {
 public:
  explicit SyncEditSession(std::function<HRESULT(TfEditCookie)> task)
      : ref_(1), task_(std::move(task)) {}

  STDMETHODIMP QueryInterface(REFIID riid, void** ppv) override {
    *ppv = nullptr;
    if (IsEqualIID(riid, IID_IUnknown) ||
        IsEqualIID(riid, IID_ITfEditSession)) {
      *ppv = static_cast<ITfEditSession*>(this);
      AddRef();
      return S_OK;
    }
    return E_NOINTERFACE;
  }
  STDMETHODIMP_(ULONG) AddRef() override {
    return InterlockedIncrement(&ref_);
  }
  STDMETHODIMP_(ULONG) Release() override {
    const ULONG cr = InterlockedDecrement(&ref_);
    if (cr == 0) delete this;
    return cr;
  }
  STDMETHODIMP DoEditSession(TfEditCookie ec) override {
    return task_ ? task_(ec) : E_FAIL;
  }

 private:
  LONG ref_;
  std::function<HRESULT(TfEditCookie)> task_;
};

HRESULT RunSyncEdit(ITfContext* ctx, TfClientId tid,
                    std::function<HRESULT(TfEditCookie)> task) {
  auto* session = new (std::nothrow) SyncEditSession(std::move(task));
  if (!session) return E_OUTOFMEMORY;
  HRESULT hr = E_FAIL;
  ctx->RequestEditSession(tid, session, TF_ES_SYNC | TF_ES_READWRITE, &hr);
  session->Release();
  return hr;
}

}  // namespace

STDMETHODIMP CTextService::QueryInterface(REFIID riid, void** ppv) {
  *ppv = nullptr;
  if (IsEqualIID(riid, IID_IUnknown)) {
    *ppv = static_cast<ITfTextInputProcessor*>(this);
  } else if (IsEqualIID(riid, IID_ITfTextInputProcessor)) {
    *ppv = static_cast<ITfTextInputProcessor*>(this);
  } else if (IsEqualIID(riid, IID_ITfKeyEventSink)) {
    *ppv = static_cast<ITfKeyEventSink*>(this);
  } else if (IsEqualIID(riid, IID_ITfCompositionSink)) {
    *ppv = static_cast<ITfCompositionSink*>(this);
  } else {
    return E_NOINTERFACE;
  }
  AddRef();
  return S_OK;
}

bool CTextService::AdviseKeyEventSink() {
  ITfKeystrokeMgr* keystroke_mgr = nullptr;
  if (FAILED(thread_mgr_->QueryInterface(IID_ITfKeystrokeMgr,
                                         reinterpret_cast<void**>(
                                             &keystroke_mgr))))
    return false;
  const HRESULT hr = keystroke_mgr->AdviseKeyEventSink(
      client_id_, static_cast<ITfKeyEventSink*>(this), FALSE);
  keystroke_mgr->Release();
  key_sink_advised_ = SUCCEEDED(hr);
  return key_sink_advised_;
}

void CTextService::UnadviseKeyEventSink() {
  if (!key_sink_advised_) return;
  ITfKeystrokeMgr* keystroke_mgr = nullptr;
  if (SUCCEEDED(thread_mgr_->QueryInterface(IID_ITfKeystrokeMgr,
                                            reinterpret_cast<void**>(
                                                &keystroke_mgr)))) {
    keystroke_mgr->UnadviseKeyEventSink(client_id_);
    keystroke_mgr->Release();
  }
  key_sink_advised_ = false;
}

bool CTextService::EnsureEngine() {
  std::string error;
  if (!engine_.EnsureReady(&error)) {
    static bool logged = false;
    if (!logged) {
      ImeLog("engine unavailable: %s (pass-through mode)", error.c_str());
      logged = true;
    }
    return false;
  }
  if (rime_session_ == 0) {
    rime_session_ = engine_.CreateSession();
    ImeLog("rime session=%zu", rime_session_);
  }
  return rime_session_ != 0;
}

STDMETHODIMP CTextService::Activate(ITfThreadMgr* ptim, TfClientId tid) {
  if (!ptim) return E_INVALIDARG;
  thread_mgr_ = ptim;
  thread_mgr_->AddRef();
  client_id_ = tid;
  char detail[64];
  _snprintf_s(detail, sizeof(detail), _TRUNCATE, "tid=%u advise=%d",
              static_cast<unsigned>(tid),
              AdviseKeyEventSink() ? 1 : 0);
  ImeLog("activate %s", detail);
  // Engine loads lazily on first key so a missing rime.dll never blocks
  // switching input methods.
  return S_OK;
}

STDMETHODIMP CTextService::Deactivate() {
  ImeLog("deactivate");
  candidate_wnd_.Hide();
  UnadviseKeyEventSink();
  if (rime_session_) {
    engine_.DestroySession(rime_session_);
    rime_session_ = 0;
  }
  if (composition_) {
    composition_->Release();
    composition_ = nullptr;
  }
  if (active_ctx_) {
    active_ctx_->Release();
    active_ctx_ = nullptr;
  }
  composition_text_.clear();
  test_processed_ = false;
  if (thread_mgr_) {
    thread_mgr_->Release();
    thread_mgr_ = nullptr;
  }
  client_id_ = 0;
  return S_OK;
}

// Measures the composition range extent in screen coordinates. The work must
// happen inside the synchronous read session (the edit cookie is only valid
// during DoEditSession).
void CTextService::RefreshCandidateWindow(ITfContext* pic) {
  RimeEngine::Snapshot snap;
  if (!engine_.GetContext(rime_session_, &snap) ||
      snap.candidates.empty()) {
    candidate_wnd_.Hide();
    return;
  }
  RECT caret = {0, 0, 0, 0};
  ITfContextView* view = nullptr;
  if (SUCCEEDED(pic->GetActiveView(&view))) {
    auto* session = new (std::nothrow) SyncEditSession(
        [&](TfEditCookie ec) -> HRESULT {
          if (!composition_) return S_FALSE;
          ITfRange* range = nullptr;
          if (FAILED(composition_->GetRange(&range))) return S_FALSE;
          view->GetTextExt(ec, range, &caret, nullptr);
          range->Release();
          return S_OK;
        });
    HRESULT hr = E_FAIL;
    pic->RequestEditSession(client_id_, session, TF_ES_SYNC | TF_ES_READ,
                            &hr);
    session->Release();
    view->Release();
  }
  HWND focus_wnd = GetFocus();
  candidate_wnd_.EnsureCreated(focus_wnd);
  candidate_wnd_.Show(snap, caret);
}

HRESULT CTextService::UpdateCompositionText(ITfContext* pic,
                                            const std::wstring& text) {
  composition_text_ = text;
  return RunSyncEdit(pic, client_id_, [&](TfEditCookie ec) -> HRESULT {
    if (!composition_) return E_FAIL;
    ITfRange* range = nullptr;
    if (FAILED(composition_->GetRange(&range))) return E_FAIL;
    const HRESULT hr = range->SetText(ec, 0, text.c_str(),
                                      static_cast<LONG>(text.size()));
    range->Release();
    return hr;
  });
}

// Opens the composition over the CURRENT SELECTION (covers both the caret
// case and the "selected text to replace" case required by AC-01).
bool CTextService::EnsureComposition(ITfContext* pic) {
  if (composition_) {
    if (active_ctx_ == pic) return true;
    EndCompositionKeepText(active_ctx_);  // context switched mid-composition
  }
  HRESULT hr = RunSyncEdit(pic, client_id_, [&](TfEditCookie ec) -> HRESULT {
    ITfRange* range = nullptr;
    TF_SELECTION sel{};
    ULONG fetched = 0;
    if (SUCCEEDED(pic->GetSelection(ec, TF_DEFAULT_SELECTION, 1, &sel,
                                    &fetched)) &&
        fetched > 0 && sel.range) {
      range = sel.range;  // the selection range (collapsed or not)
    } else {
      ITfInsertAtSelection* insert_at_selection = nullptr;
      if (FAILED(pic->QueryInterface(IID_ITfInsertAtSelection,
                                     reinterpret_cast<void**>(
                                         &insert_at_selection))))
        return E_FAIL;
      const HRESULT hr_ins = insert_at_selection->InsertTextAtSelection(
          ec, 0, L"", 0, &range);
      insert_at_selection->Release();
      if (FAILED(hr_ins)) return hr_ins;
    }
    ITfContextComposition* context_composition = nullptr;
    if (FAILED(pic->QueryInterface(IID_ITfContextComposition,
                                   reinterpret_cast<void**>(
                                       &context_composition)))) {
      range->Release();
      return E_FAIL;
    }
    const HRESULT hr_start = context_composition->StartComposition(
        ec, range, static_cast<ITfCompositionSink*>(this), &composition_);
    context_composition->Release();
    range->Release();
    if (SUCCEEDED(hr_start) && composition_) {
      active_ctx_ = pic;
      active_ctx_->AddRef();
    }
    return hr_start;
  });
  if (FAILED(hr)) ImeLog("StartComposition failed hr=0x%08lx",
                         static_cast<unsigned long>(hr));
  return SUCCEEDED(hr) && composition_ != nullptr;
}

HRESULT CTextService::FinalizeComposition(ITfContext* pic,
                                          const std::wstring& text) {
  composition_text_.clear();
  candidate_wnd_.Hide();
  if (!composition_) {
    // No open composition (e.g. Rime committed without composing): insert.
    return RunSyncEdit(pic, client_id_, [&](TfEditCookie ec) -> HRESULT {
      ITfInsertAtSelection* insert_at_selection = nullptr;
      if (FAILED(pic->QueryInterface(IID_ITfInsertAtSelection,
                                     reinterpret_cast<void**>(
                                         &insert_at_selection))))
        return E_FAIL;
      ITfRange* range = nullptr;
      const HRESULT hr = insert_at_selection->InsertTextAtSelection(
          ec, 0, text.c_str(), static_cast<LONG>(text.size()), &range);
      if (SUCCEEDED(hr) && range) range->Release();
      insert_at_selection->Release();
      return hr;
    });
  }
  ITfComposition* comp = composition_;
  composition_ = nullptr;
  ITfContext* ctx = active_ctx_;
  active_ctx_ = nullptr;
  const HRESULT hr = RunSyncEdit(pic, client_id_, [&](TfEditCookie ec) {
    ITfRange* range = nullptr;
    if (FAILED(comp->GetRange(&range))) return E_FAIL;
    const HRESULT hr_set = range->SetText(ec, 0, text.c_str(),
                                          static_cast<LONG>(text.size()));
    comp->EndComposition(ec);
    range->Release();
    return hr_set;
  });
  comp->Release();
  if (ctx) ctx->Release();
  return hr;
}

void CTextService::EndCompositionKeepText(ITfContext* pic) {
  if (!composition_) return;
  ITfComposition* comp = composition_;
  composition_ = nullptr;
  ITfContext* ctx = active_ctx_;
  active_ctx_ = nullptr;
  composition_text_.clear();
  RunSyncEdit(pic, client_id_, [&](TfEditCookie ec) {
    ITfRange* range = nullptr;
    if (SUCCEEDED(comp->GetRange(&range))) {
      range->Release();
    }
    comp->EndComposition(ec);
    return S_OK;
  });
  comp->Release();
  if (ctx) ctx->Release();
}

void CTextService::ApplyCompositionState(
    ITfContext* pic, bool has_commit, const std::string& commit_utf8,
    const RimeEngine::Snapshot& snap) {
  const std::wstring preedit = Utf8ToUtf16(snap.preedit_utf8);
  if (has_commit && !commit_utf8.empty()) {
    // Close the current composition with the committed text (or insert at
    // the selection when no composition is open); if Rime still holds
    // preedit, a fresh composition opens below.
    FinalizeComposition(pic, Utf8ToUtf16(commit_utf8));
    if (preedit.empty()) return;
  }
  if (!preedit.empty()) {
    if (!EnsureComposition(pic)) {
      candidate_wnd_.Hide();
      return;
    }
    if (preedit != composition_text_) UpdateCompositionText(pic, preedit);
    RefreshCandidateWindow(pic);
    return;
  }
  // Engine consumed the key but produced neither preedit nor commit
  // (composition cleared): close the composition, keep its text.
  EndCompositionKeepText(pic);
  candidate_wnd_.Hide();
}

bool CTextService::ProcessAndApplyKey(ITfContext* pic, WPARAM wParam) {
  if (!EnsureEngine()) return false;
  const int keysym = AutoIMEVkToKeysym(wParam, 0);
  if (keysym == 0) return false;
  if (!engine_.ProcessKey(rime_session_, keysym)) return false;

  bool has_commit = false;
  const std::string commit = engine_.TakeCommit(rime_session_, &has_commit);
  RimeEngine::Snapshot snap;
  if (!engine_.GetContext(rime_session_, &snap)) snap.preedit_utf8.clear();
  ApplyCompositionState(pic, has_commit, commit, snap);
  return true;
}

STDMETHODIMP CTextService::OnSetFocus(BOOL fForeground) {
  ImeLog("OnSetFocus foreground=%d", fForeground);
  if (!fForeground) candidate_wnd_.Hide();
  return S_OK;
}

STDMETHODIMP CTextService::OnTestKeyDown(ITfContext* pic, WPARAM wParam,
                                         LPARAM, BOOL* pfEaten) {
  *pfEaten = FALSE;
  if (!EnsureEngine()) return S_OK;  // pass-through while engine unavailable
  const bool consumed = ProcessAndApplyKey(pic, wParam);
  test_processed_ = consumed;
  *pfEaten = consumed ? TRUE : FALSE;
  return S_OK;
}

STDMETHODIMP CTextService::OnKeyDown(ITfContext* pic, WPARAM wParam, LPARAM,
                                     BOOL* pfEaten) {
  if (test_processed_) {
    // The key was already processed in OnTestKeyDown; do not double-apply.
    test_processed_ = false;
    *pfEaten = TRUE;
    return S_OK;
  }
  *pfEaten = ProcessAndApplyKey(pic, wParam) ? TRUE : FALSE;
  return S_OK;
}

STDMETHODIMP CTextService::OnTestKeyUp(ITfContext*, WPARAM, LPARAM,
                                       BOOL* pfEaten) {
  *pfEaten = FALSE;
  return S_OK;
}

STDMETHODIMP CTextService::OnKeyUp(ITfContext*, WPARAM, LPARAM,
                                   BOOL* pfEaten) {
  *pfEaten = FALSE;
  return S_OK;
}

STDMETHODIMP CTextService::OnPreservedKey(ITfContext*, REFGUID,
                                          BOOL* pfEaten) {
  *pfEaten = FALSE;
  return S_OK;
}

STDMETHODIMP CTextService::OnCompositionTerminated(
    TfEditCookie, ITfComposition* pComposition) {
  ImeLog("OnCompositionTerminated");
  // Focus or selection moved away: end our state cleanly. The text already
  // in the terminated composition stays visible in ITS document (never
  // written to another window); the engine session is reset.
  if (composition_) {
    composition_->Release();
    composition_ = nullptr;
  }
  (void)pComposition;  // TSF has already terminated it.
  if (active_ctx_) {
    active_ctx_->Release();
    active_ctx_ = nullptr;
  }
  composition_text_.clear();
  test_processed_ = false;
  candidate_wnd_.Hide();
  if (rime_session_) engine_.ClearComposition(rime_session_);
  return S_OK;
}
