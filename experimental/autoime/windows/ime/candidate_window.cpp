// AutoIME - native Win32 candidate window implementation (PLAN-069 T-03).
#include "candidate_window.h"

#include "ime.h"

namespace {
constexpr wchar_t kWndClass[] = L"AutoIMECandidateWindow";
constexpr int kRowH = 28;
constexpr int kPadX = 10;
constexpr int kPadY = 6;
}  // namespace

CandidateWindow::~CandidateWindow() {
  if (wnd_) DestroyWindow(wnd_);
  if (font_) DeleteObject(font_);
}

bool CandidateWindow::EnsureCreated(HWND owner) {
  if (wnd_) {
    // Re-owner to the current editor so the popup follows the focus host.
    SetWindowLongPtrW(wnd_, GWLP_HWNDPARENT,
                      reinterpret_cast<LONG_PTR>(owner));
    return true;
  }
  WNDCLASSW wc = {};
  wc.lpfnWndProc = &CandidateWindow::WndProc;
  wc.hInstance = g_hInst;
  wc.hCursor = LoadCursor(nullptr, IDC_ARROW);
  wc.hbrBackground = CreateSolidBrush(RGB(0xFA, 0xFA, 0xFA));
  wc.lpszClassName = kWndClass;
  RegisterClassW(&wc);

  font_ = CreateFontW(-17, 0, 0, 0, FW_NORMAL, FALSE, FALSE, FALSE,
                      DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS,
                      CLEARTYPE_QUALITY, DEFAULT_PITCH | FF_DONTCARE,
                      L"Microsoft YaHei UI");

  wnd_ = CreateWindowExW(
      WS_EX_TOPMOST | WS_EX_NOACTIVATE | WS_EX_TOOLWINDOW, kWndClass, L"",
      WS_POPUP, 0, 0, 10, 10, owner, nullptr, g_hInst, this);
  return wnd_ != nullptr;
}

void CandidateWindow::Layout(const RimeEngine::Snapshot& snap,
                             const RECT& caret) {
  lines_.clear();
  numbers_.clear();
  highlighted_ = snap.highlighted;
  last_page_ = snap.is_last_page;
  page_no_ = snap.page_no;
  int max_w = 0;
  for (size_t i = 0; i < snap.candidates.size(); ++i) {
    lines_.push_back(snap.candidates[i]);
    numbers_.push_back(static_cast<int>(i) + 1);
  }
  HDC dc = GetDC(wnd_);
  HFONT old = static_cast<HFONT>(SelectObject(dc, font_));
  SIZE sz = {};
  for (const std::wstring& line : lines_) {
    GetTextExtentPoint32W(dc, line.c_str(),
                          static_cast<int>(line.size()), &sz);
    if (sz.cx > max_w) max_w = sz.cx;
  }
  SelectObject(dc, old);
  ReleaseDC(wnd_, dc);

  const int w = max_w + kPadX * 2 + 22;  // room for the number column
  const int h = static_cast<int>(lines_.size()) * kRowH + kPadY * 2;
  int x = caret.left;
  int y = caret.bottom + 2;
  HMONITOR mon = MonitorFromPoint({caret.left, caret.top},
                                  MONITOR_DEFAULTTONEAREST);
  MONITORINFO mi = {sizeof(mi)};
  GetMonitorInfoW(mon, &mi);
  if (x + w > mi.rcWork.right) x = mi.rcWork.right - w;
  if (y + h > mi.rcWork.bottom) y = caret.top - h - 2;  // flip above caret
  if (y < mi.rcWork.top) y = mi.rcWork.top;
  SetWindowPos(wnd_, HWND_TOPMOST, x, y, w, h,
               SWP_NOACTIVATE | SWP_SHOWWINDOW);
  visible_ = true;
}

void CandidateWindow::Show(const RimeEngine::Snapshot& snap,
                           const RECT& caret) {
  if (!wnd_ || snap.candidates.empty()) {
    Hide();
    return;
  }
  Layout(snap, caret);
  InvalidateRect(wnd_, nullptr, TRUE);
}

void CandidateWindow::Hide() {
  if (wnd_ && visible_) ShowWindow(wnd_, SW_HIDE);
  visible_ = false;
}

void CandidateWindow::Paint(HDC dc) {
  RECT rc;
  GetClientRect(wnd_, &rc);
  FillRect(dc, &rc, reinterpret_cast<HBRUSH>(GetClassLongPtrW(
                        wnd_, GCLP_HBRBACKGROUND)));
  SetBkMode(dc, TRANSPARENT);
  SelectObject(dc, font_);
  wchar_t num[8];
  int y = kPadY;
  const HBRUSH highlight = CreateSolidBrush(RGB(0x2B, 0x6C, 0xF0));
  for (size_t i = 0; i < lines_.size(); ++i) {
    if (static_cast<int>(i) == highlighted_) {
      RECT row = {0, y, rc.right, y + kRowH};
      FillRect(dc, &row, highlight);
      SetTextColor(dc, RGB(0xFF, 0xFF, 0xFF));
    } else {
      SetTextColor(dc, RGB(0x20, 0x20, 0x20));
    }
    wsprintfW(num, L"%d", numbers_[i]);
    TextOutW(dc, kPadX, y + 4, num, lstrlenW(num));
    TextOutW(dc, kPadX + 22, y + 4, lines_[i].c_str(),
             static_cast<int>(lines_[i].size()));
    y += kRowH;
  }
  DeleteObject(highlight);
}

LRESULT CALLBACK CandidateWindow::WndProc(HWND wnd, UINT msg, WPARAM wparam,
                                          LPARAM lparam) {
  auto* self = reinterpret_cast<CandidateWindow*>(
      GetWindowLongPtrW(wnd, GWLP_USERDATA));
  switch (msg) {
    case WM_NCCREATE: {
      auto* cs = reinterpret_cast<CREATESTRUCTW*>(lparam);
      SetWindowLongPtrW(wnd, GWLP_USERDATA,
                        reinterpret_cast<LONG_PTR>(cs->lpCreateParams));
      return DefWindowProcW(wnd, msg, wparam, lparam);
    }
    case WM_PAINT: {
      PAINTSTRUCT ps;
      HDC dc = BeginPaint(wnd, &ps);
      if (self) self->Paint(dc);
      EndPaint(wnd, &ps);
      return 0;
    }
    case WM_MOUSEACTIVATE:
      return MA_NOACTIVATE;  // never steal editor focus
    default:
      return DefWindowProcW(wnd, msg, wparam, lparam);
  }
}
