// AutoIME - native Win32 candidate window (PLAN-069 T-03, PLAN-070 T-03).
// Popup that never takes focus (WS_EX_NOACTIVATE, owner = editor window), so
// the editor keeps keyboard focus while candidates are visible. It renders the
// engine's real selection labels, per-monitor DPI metrics and supports mouse
// selection: a click carries the page-local index and the page version it was
// shown with, so a stale click cannot commit a different candidate.
#ifndef AUTOIME_CANDIDATE_WINDOW_H_
#define AUTOIME_CANDIDATE_WINDOW_H_

#include <cstdint>
#include <functional>
#include <string>
#include <vector>

#include <windows.h>

#include "rime_engine.h"

class CandidateWindow {
 public:
  using SelectHandler = std::function<void(int page_index, uint64_t version)>;

  ~CandidateWindow();
  // Creates the window (once per process) owned by `owner`.
  bool EnsureCreated(HWND owner);
  // Shows the current page's candidates anchored at `caret` (screen coords).
  void Show(const RimeEngine::Snapshot& snap, const RECT& caret);
  void Hide();
  bool visible() const { return wnd_ != nullptr && visible_; }
  void SetSelectHandler(SelectHandler handler) { handler_ = std::move(handler); }
  uint64_t version() const { return version_; }

 private:
  static LRESULT CALLBACK WndProc(HWND wnd, UINT msg, WPARAM wparam,
                                  LPARAM lparam);
  void Layout(const RimeEngine::Snapshot& snap, const RECT& caret);
  void Paint(HDC dc);
  void OnClick(int y);

  HWND wnd_ = nullptr;
  HFONT font_ = nullptr;
  bool visible_ = false;
  std::vector<std::wstring> lines_;   // one line per candidate row
  std::vector<std::wstring> labels_;  // engine-provided selection labels
  int highlighted_ = 0;
  bool last_page_ = true;
  int page_no_ = 0;
  uint64_t version_ = 0;
  int row_h_ = 28;
  int pad_x_ = 10;
  int pad_y_ = 6;
  int label_w_ = 18;
  SelectHandler handler_;
};

#endif  // AUTOIME_CANDIDATE_WINDOW_H_
