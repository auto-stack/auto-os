// AutoIME - native Win32 candidate window (PLAN-069 T-03).
// Popup that never takes focus (WS_EX_NOACTIVATE, owner = editor window),
// so the editor keeps keyboard focus while candidates are visible.
#ifndef AUTOIME_CANDIDATE_WINDOW_H_
#define AUTOIME_CANDIDATE_WINDOW_H_

#include <string>
#include <vector>

#include <windows.h>

#include "rime_engine.h"

class CandidateWindow {
 public:
  ~CandidateWindow();
  // Creates the window (once per process) owned by `owner`.
  bool EnsureCreated(HWND owner);
  // Shows candidates anchored at `caret` (screen coords, from GetTextExt).
  void Show(const RimeEngine::Snapshot& snap, const RECT& caret);
  void Hide();
  bool visible() const { return wnd_ != nullptr && visible_; }

 private:
  static LRESULT CALLBACK WndProc(HWND wnd, UINT msg, WPARAM wparam,
                                  LPARAM lparam);
  void Layout(const RimeEngine::Snapshot& snap, const RECT& caret);
  void Paint(HDC dc);

  HWND wnd_ = nullptr;
  HFONT font_ = nullptr;
  bool visible_ = false;
  std::vector<std::wstring> lines_;   // one line per candidate row
  std::vector<int> numbers_;          // selection number per row
  int highlighted_ = 0;
  bool last_page_ = true;
  int page_no_ = 0;
};

#endif  // AUTOIME_CANDIDATE_WINDOW_H_
