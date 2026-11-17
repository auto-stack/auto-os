// AutoIME TSF text service - key routing, composition, candidates.
//
// PLAN-070 T-01 rewires the callbacks through KeyRouter:
//   * OnTestKeyDown only DECIDES (no engine mutation, no commit, no learning);
//   * OnKeyDown performs the single engine operation for that physical key,
//     reusing the decision cached by a preceding Test;
//   * repeated Test callbacks, Down-only hosts, auto-repeat and late callbacks
//     from a previous focus generation are all handled explicitly;
//   * Ctrl/Alt/Win shortcuts (including AltGr) stay with the host;
//   * password/sensitive input scopes and host-disabled contexts pass through;
//   * a single Shift click switches Chinese/English mode.
// Diagnostics are payload-free (stage/code/counters/timings only).
#include "ime.h"

#include <inputscope.h>

#include <functional>
#include <new>

#include "diag.h"

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

HRESULT RunSyncEdit(ITfContext* ctx, TfClientId tid, DWORD flags,
                    std::function<HRESULT(TfEditCookie)> task) {
  auto* session = new (std::nothrow) SyncEditSession(std::move(task));
  if (!session) return E_OUTOFMEMORY;
  HRESULT hr = E_FAIL;
  ctx->RequestEditSession(tid, session, TF_ES_SYNC | flags, &hr);
  session->Release();
  return hr;
}

HRESULT RunSyncEdit(ITfContext* ctx, TfClientId tid,
                    std::function<HRESULT(TfEditCookie)> task) {
  return RunSyncEdit(ctx, tid, TF_ES_READWRITE, std::move(task));
}

// GUID_PROP_INPUTSCOPE as published by InputScope.h. The SDK header only
// declares it; linking would require a symbol the TSF runtime does not export
// from a dedicated LIB, so keep the literal value here.
const GUID kPropInputScope = {
    0x1713dd5a, 0x68e7, 0x4a5b, {0x9a, 0xf6, 0x59, 0x2a, 0x59, 0x5c, 0x77,
                                0x8d}};

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
  // fForeground=TRUE: same as the MS TSF sample and Weasel. FALSE was
  // observed to receive no key events at all.
  const HRESULT hr = keystroke_mgr->AdviseKeyEventSink(
      client_id_, static_cast<ITfKeyEventSink*>(this), TRUE);
  keystroke_mgr->Release();
  key_sink_advised_ = SUCCEEDED(hr);
  ImeLog("AdviseKeyEventSink hr=0x%08lx foreground=1",
         static_cast<unsigned long>(hr));
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
  if (engine_load_failed_) return false;
  diag::Init();
  std::string error;
  if (!engine_.EnsureReady(&error)) {
    if (!engine_load_failed_) {
      ImeLog("engine unavailable: %s (pass-through mode)", error.c_str());
      diag::Failure("engine", E_FAIL);
      engine_load_failed_ = true;
    }
    return false;
  }
  if (rime_session_ == 0) {
    rime_session_ = engine_.CreateSession();
    if (rime_session_) {
      bool disabled = false;
      engine_.GetStatus(rime_session_, &ascii_mode_, &disabled);
      engine_composing_ = false;
      ImeLog("rime session=%zu ascii=%d", rime_session_, ascii_mode_ ? 1 : 0);
    }
  }
  return rime_session_ != 0;
}

STDMETHODIMP CTextService::Activate(ITfThreadMgr* ptim, TfClientId tid) {
  if (!ptim) return E_INVALIDARG;
  diag::Init();
  thread_mgr_ = ptim;
  thread_mgr_->AddRef();
  client_id_ = tid;
  router_.SetTarget(this);
  router_.OnFocus(generation_, true);
  char detail[64];
  _snprintf_s(detail, sizeof(detail), _TRUNCATE, "tid=%u advise=%d",
              static_cast<unsigned>(tid),
              AdviseKeyEventSink() ? 1 : 0);
  ImeLog("activate %s", detail);
  diag::Event("service", "activate");
  // Engine loads lazily on first key so a missing rime.dll never blocks
  // switching input methods.
  return S_OK;
}

STDMETHODIMP CTextService::Deactivate() {
  ImeLog("deactivate");
  diag::Event("service", "deactivate");
  candidate_wnd_.Hide();
  UnadviseKeyEventSink();
  if (composition_) {
    composition_->Release();
    composition_ = nullptr;
  }
  if (active_ctx_) {
    active_ctx_->Release();
    active_ctx_ = nullptr;
  }
  composition_text_.clear();
  engine_composing_ = false;
  if (rime_session_) {
    engine_.DestroySession(rime_session_);
    rime_session_ = 0;
  }
  current_ctx_ = nullptr;
  router_.SetTarget(nullptr);
  if (thread_mgr_) {
    thread_mgr_->Release();
    thread_mgr_ = nullptr;
  }
  client_id_ = 0;
  return S_OK;
}

bool CTextService::IsSensitiveContext(ITfContext* pic) {
  if (!pic || !thread_mgr_) return false;
  // 1. The host asked TSF to disable the IME (e.g. a password dialog).
  {
    ITfCompartmentMgr* mgr = nullptr;
    if (SUCCEEDED(thread_mgr_->QueryInterface(
            IID_ITfCompartmentMgr, reinterpret_cast<void**>(&mgr))) &&
        mgr) {
      ITfCompartment* comp = nullptr;
      if (SUCCEEDED(mgr->GetCompartment(GUID_COMPARTMENT_KEYBOARD_DISABLED,
                                        &comp)) &&
          comp) {
        VARIANT var;
        VariantInit(&var);
        bool disabled = false;
        if (SUCCEEDED(comp->GetValue(&var))) {
          if (var.vt == VT_I4) disabled = var.lVal != 0;
          VariantClear(&var);
        }
        comp->Release();
        mgr->Release();
        if (disabled) return true;
      } else {
        mgr->Release();
      }
    }
  }
  // 2. The caret sits in a password/sensitive input scope. Windows reports it
  // through GUID_PROP_INPUTSCOPE on the current selection range.
  bool sensitive = false;
  RunSyncEdit(pic, client_id_, TF_ES_READ, [&](TfEditCookie ec) -> HRESULT {
    ITfProperty* prop = nullptr;
    if (FAILED(pic->GetProperty(kPropInputScope, &prop)) || !prop)
      return S_FALSE;
    TF_SELECTION sel{};
    ULONG fetched = 0;
    if (FAILED(pic->GetSelection(ec, TF_DEFAULT_SELECTION, 1, &sel,
                                 &fetched)) ||
        fetched == 0 || !sel.range) {
      prop->Release();
      return S_FALSE;
    }
    VARIANT var;
    VariantInit(&var);
    const HRESULT hr = prop->GetValue(ec, sel.range, &var);
    sel.range->Release();
    if (SUCCEEDED(hr)) {
      IUnknown* unknown = nullptr;
      if (var.vt == VT_UNKNOWN) unknown = var.punkVal;
      if (var.vt == VT_DISPATCH && var.pdispVal)
        unknown = static_cast<IUnknown*>(var.pdispVal);
      if (unknown) {
        ITfInputScope* scope = nullptr;
        if (SUCCEEDED(unknown->QueryInterface(
                IID_ITfInputScope, reinterpret_cast<void**>(&scope))) &&
            scope) {
          InputScope* scopes = nullptr;
          UINT count = 0;
          if (SUCCEEDED(scope->GetInputScopes(&scopes, &count)) && scopes) {
            for (UINT i = 0; i < count; ++i) {
              if (scopes[i] == IS_PASSWORD ||
                  scopes[i] == IS_NUMERIC_PASSWORD)
                sensitive = true;
            }
            CoTaskMemFree(scopes);
          }
          scope->Release();
        }
      }
      VariantClear(&var);
    }
    prop->Release();
    return S_OK;
  });
  return sensitive;
}

KeyEvent CTextService::MakeKeyEvent(WPARAM wParam, LPARAM lParam) const {
  KeyEvent key;
  key.vk = wParam;
  key.mods.shift = (GetKeyState(VK_SHIFT) & 0x8000) != 0;
  key.mods.ctrl = (GetKeyState(VK_CONTROL) & 0x8000) != 0;
  key.mods.alt = (GetKeyState(VK_MENU) & 0x8000) != 0;
  key.mods.win = (GetKeyState(VK_LWIN) & 0x8000) != 0 ||
                 (GetKeyState(VK_RWIN) & 0x8000) != 0;
  key.repeat = (lParam & (1 << 30)) != 0;
  return key;
}

bool CTextService::Composing() { return engine_composing_; }
bool CTextService::AsciiMode() { return ascii_mode_; }
bool CTextService::EngineReady() { return EnsureEngine(); }
bool CTextService::Sensitive() { return IsSensitiveContext(current_ctx_); }

bool CTextService::ProcessKey(int keysym, int rime_modifiers) {
  if (!EnsureEngine()) return false;
  LARGE_INTEGER start{}, end{}, freq{};
  QueryPerformanceCounter(&start);
  const bool consumed =
      engine_.ProcessKey(rime_session_, keysym, rime_modifiers);
  bool has_commit = false;
  const std::string commit = engine_.TakeCommit(rime_session_, &has_commit);
  RimeEngine::Snapshot snap;
  if (!engine_.GetContext(rime_session_, &snap)) {
    snap = RimeEngine::Snapshot{};
  }
  engine_composing_ = snap.is_composing;
  bool disabled = false;
  engine_.GetStatus(rime_session_, &ascii_mode_, &disabled);
  ApplyCompositionState(current_ctx_ ? current_ctx_ : active_ctx_, has_commit,
                        commit, snap);
  QueryPerformanceCounter(&end);
  QueryPerformanceFrequency(&freq);
  const long long us = freq.QuadPart
                           ? (end.QuadPart - start.QuadPart) * 1000000 /
                                 freq.QuadPart
                           : 0;
  diag::Count("key", "latency_us", us);
  return consumed;
}

void CTextService::ToggleMode() {
  if (!EnsureEngine()) return;
  ascii_mode_ = !ascii_mode_;
  engine_.SetOption(rime_session_, "ascii_mode", ascii_mode_);
  diag::Event("mode", ascii_mode_ ? "ascii" : "chinese");
}

void CTextService::CancelComposition() {
  if (rime_session_) engine_.ClearComposition(rime_session_);
  engine_composing_ = false;
  candidate_wnd_.Hide();
  ITfContext* ctx = active_ctx_ ? active_ctx_ : current_ctx_;
  if (composition_ && ctx) DiscardComposition(ctx);
  diag::Event("composition", "cancel");
}

// Measures the composition range extent in screen coordinates. The work must
// happen inside the synchronous read session (the edit cookie is only valid
// during DoEditSession).
void CTextService::RefreshCandidateWindow(ITfContext* pic) {
  if (!pic) return;
  RimeEngine::Snapshot snap;
  const bool have_composition =
      engine_.GetContext(rime_session_, &snap) && !snap.candidates.empty();
  if (!have_composition) {
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
          BOOL clipped = FALSE;
          const HRESULT hr_ext =
              view->GetTextExt(ec, range, &caret, &clipped);
          ImeLog("GetTextExt hr=0x%08lx clipped=%d rect=(%ld,%ld,%ld,%ld)",
                 static_cast<unsigned long>(hr_ext), clipped ? 1 : 0,
                 static_cast<long>(caret.left), static_cast<long>(caret.top),
                 static_cast<long>(caret.right),
                 static_cast<long>(caret.bottom));
          range->Release();
          return S_OK;
        });
    HRESULT hr = E_FAIL;
    pic->RequestEditSession(client_id_, session, TF_ES_SYNC | TF_ES_READ, &hr);
    session->Release();
    view->Release();
  }
  if (caret.left == 0 && caret.top == 0 && caret.right == 0 &&
      caret.bottom == 0) {
    // fallback: the host application's caret rectangle (client coords of the
    // focused window -> screen)
    GUITHREADINFO gi{};
    gi.cbSize = sizeof(gi);
    if (GetGUIThreadInfo(GetCurrentThreadId(), &gi) && gi.hwndCaret) {
      caret = gi.rcCaret;
      POINT p{caret.left, caret.top};
      ScreenToClient(gi.hwndCaret, &p);
      POINT tl{p.x, p.y};
      ClientToScreen(gi.hwndCaret, &tl);
      POINT br{caret.right, caret.bottom};
      ClientToScreen(gi.hwndCaret, &br);
      caret.left = tl.x;
      caret.top = tl.y;
      caret.right = br.x;
      caret.bottom = br.y;
      ImeLog("caret fallback via GUITHREADINFO");
    } else {
      ImeLog("caret fallback unavailable (no hwndCaret)");
    }
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
    if (SUCCEEDED(hr)) {
      // keep the caret at the composition end while composing
      range->Collapse(ec, TF_ANCHOR_END);
      TF_SELECTION sel{};
      sel.range = range;
      sel.style.ase = TF_AE_END;
      sel.style.fInterimChar = FALSE;
      pic->SetSelection(ec, 1, &sel);
    }
    range->Release();
    return hr;
  });
}

// Opens the composition over the CURRENT SELECTION (covers both the caret
// case and the "selected text to replace" case required by AC-01).
bool CTextService::EnsureComposition(ITfContext* pic) {
  if (composition_) {
    if (active_ctx_ == pic) return true;
    DiscardComposition(active_ctx_ ? active_ctx_ : pic);
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
      ImeLog("StartComposition ok comp=%p", (void*)composition_);
    } else {
      ImeLog("StartComposition FAILED hr=0x%08lx",
             static_cast<unsigned long>(hr_start));
    }
    return hr_start;
  });
  if (FAILED(hr))
    ImeLog("StartComposition session failed hr=0x%08lx",
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
    // park the caret AFTER the committed text
    if (SUCCEEDED(hr_set)) {
      range->Collapse(ec, TF_ANCHOR_END);
      TF_SELECTION sel{};
      sel.range = range;
      sel.style.ase = TF_AE_END;
      sel.style.fInterimChar = FALSE;
      pic->SetSelection(ec, 1, &sel);
    }
    comp->EndComposition(ec);
    range->Release();
    return hr_set;
  });
  comp->Release();
  if (ctx) ctx->Release();
  return hr;
}

// Drops the open composition without committing its text. The engine session
// is cleared separately by the caller. T-02 replaces this with the reversible
// path that restores the original selection.
void CTextService::DiscardComposition(ITfContext* pic) {
  if (!composition_) return;
  ITfComposition* comp = composition_;
  composition_ = nullptr;
  ITfContext* ctx = active_ctx_;
  active_ctx_ = nullptr;
  composition_text_.clear();
  if (pic) {
    RunSyncEdit(pic, client_id_, [&](TfEditCookie ec) {
      ITfRange* range = nullptr;
      if (SUCCEEDED(comp->GetRange(&range))) {
        range->SetText(ec, 0, L"", 0);  // discard the composition text
        comp->EndComposition(ec);
        range->Release();
      }
      return S_OK;
    });
  }
  comp->Release();
  if (ctx) ctx->Release();
}

void CTextService::ApplyCompositionState(
    ITfContext* pic, bool has_commit, const std::string& commit_utf8,
    const RimeEngine::Snapshot& snap) {
  if (!pic) return;
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
  // (composition cancelled, e.g. Esc): remove the composition text so the
  // document returns to its pre-edit state.
  DiscardComposition(pic);
  candidate_wnd_.Hide();
}

STDMETHODIMP CTextService::OnSetFocus(BOOL fForeground) {
  ImeLog("OnSetFocus foreground=%d", fForeground);
  if (!fForeground) {
    candidate_wnd_.Hide();
    // Revert an unfinished composition before the old context turns stale.
    if (composition_) CancelComposition();
    if (rime_session_) engine_.ClearComposition(rime_session_);
    engine_composing_ = false;
    ++generation_;
    router_.OnFocus(generation_, false);
  } else {
    router_.OnFocus(generation_, true);
  }
  return S_OK;
}

STDMETHODIMP CTextService::OnTestKeyDown(ITfContext* pic, WPARAM wParam,
                                         LPARAM lParam, BOOL* pfEaten) {
  *pfEaten = FALSE;
  if (!pic) return S_OK;
  current_ctx_ = pic;
  const KeyEvent key = MakeKeyEvent(wParam, lParam);
  const RouteDecision decision = router_.OnTestKeyDown(key, generation_);
  *pfEaten = decision.eat ? TRUE : FALSE;
  return S_OK;
}

STDMETHODIMP CTextService::OnKeyDown(ITfContext* pic, WPARAM wParam,
                                     LPARAM lParam, BOOL* pfEaten) {
  *pfEaten = FALSE;
  if (!pic) return S_OK;
  current_ctx_ = pic;
  const KeyEvent key = MakeKeyEvent(wParam, lParam);
  const RouteDecision decision = router_.OnKeyDown(key, generation_);
  *pfEaten = decision.eat ? TRUE : FALSE;
  return S_OK;
}

STDMETHODIMP CTextService::OnTestKeyUp(ITfContext*, WPARAM, LPARAM,
                                       BOOL* pfEaten) {
  *pfEaten = FALSE;
  return S_OK;
}

STDMETHODIMP CTextService::OnKeyUp(ITfContext* pic, WPARAM wParam,
                                   LPARAM lParam, BOOL* pfEaten) {
  *pfEaten = FALSE;
  if (!pic) return S_OK;
  current_ctx_ = pic;
  const KeyEvent key = MakeKeyEvent(wParam, lParam);
  router_.OnKeyUp(key, generation_);
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
  // The host (or another service) ended the composition. Whatever the host
  // already did to the document is authoritative: only release our state and
  // retire the generation so late callbacks cannot write into the old target.
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
  candidate_wnd_.Hide();
  engine_composing_ = false;
  if (rime_session_) engine_.ClearComposition(rime_session_);
  ++generation_;
  router_.OnFocus(generation_, false);
  diag::Event("composition", "terminated_by_host");
  return S_OK;
}
