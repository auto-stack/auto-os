// AutoIME - native Win32 candidate window implementation (PLAN-069 T-03,
// PLAN-070 T-03: real labels, page version, DPI metrics, click selection).
#include "candidate_window.h"

#include "diag.h"
#include "ime.h"

namespace {
constexpr wchar_t kWndClass[] = L"AutoIMECandidateWindow";
// Base metrics at 96 DPI; scaled per monitor when the window is created.
constexpr int kRowH96 = 28;
constexpr int kPadX96 = 10;
constexpr int kPadY96 = 6;

int Scale(int value, UINT dpi) {
  return MulDiv(value, static_cast<int>(dpi ? dpi : 96), 96);
}
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

  font_ = CreateFontW(-Scale(17, GetDpiForSystem()), 0, 0, 0, FW_NORMAL, FALSE,
                      FALSE, FALSE, DEFAULT_CHARSET, OUT_DEFAULT_PRECIS,
                      CLIP_DEFAULT_PRECIS, CLEARTYPE_QUALITY,
                      DEFAULT_PITCH | FF_DONTCARE, L"Microsoft YaHei UI");

  wnd_ = CreateWindowExW(
      WS_EX_TOPMOST | WS_EX_NOACTIVATE | WS_EX_TOOLWINDOW, kWndClass, L"",
      WS_POPUP, 0, 0, 10, 10, owner, nullptr, g_hInst, this);
  return wnd_ != nullptr;
}

void CandidateWindow::Layout(const RimeEngine::Snapshot& snap,
                             const RECT& caret) {
  lines_.clear();
  labels_.clear();
  highlighted_ = snap.highlighted;
  last_page_ = snap.is_last_page;
  page_no_ = snap.page_no;
  const UINT dpi = wnd_ ? GetDpiForWindow(wnd_) : 0;
  const UINT effective_dpi = dpi ? dpi : 96;
  row_h_ = Scale(kRowH96, effective_dpi);
  pad_x_ = Scale(kPadX96, effective_dpi);
  pad_y_ = Scale(kPadY96, effective_dpi);
  for (size_t i = 0; i < snap.candidates.size(); ++i) {
    lines_.push_back(snap.candidates[i]);
    if (i < snap.labels.size() && !snap.labels[i].empty())
      labels_.push_back(snap.labels[i]);
    else
      labels_.push_back(std::to_wstring(i + 1));
  }
  HDC dc = GetDC(wnd_);
  HFONT old = static_cast<HFONT>(SelectObject(dc, font_));
  SIZE sz = {};
  int max_w = 0;
  int max_label_w = 0;
  for (const std::wstring& line : lines_) {
    GetTextExtentPoint32W(dc, line.c_str(), static_cast<int>(line.size()),
                          &sz);
    if (sz.cx > max_w) max_w = sz.cx;
  }
  for (const std::wstring& label : labels_) {
    GetTextExtentPoint32W(dc, label.c_str(), static_cast<int>(label.size()),
                          &sz);
    if (sz.cx > max_label_w) max_label_w = sz.cx;
  }
  SelectObject(dc, old);
  ReleaseDC(wnd_, dc);
  label_w_ = max_label_w + Scale(8, effective_dpi);

  const int w = max_w + pad_x_ * 2 + label_w_;
  const int h = static_cast<int>(lines_.size()) * row_h_ + pad_y_ * 2;
  int x = caret.left;
  int y = caret.bottom + Scale(2, effective_dpi);
  HMONITOR mon = MonitorFromPoint({caret.left, caret.top},
                                  MONITOR_DEFAULTTONEAREST);
  MONITORINFO mi = {sizeof(mi)};
  GetMonitorInfoW(mon, &mi);
  if (x + w > mi.rcWork.right) x = mi.rcWork.right - w;
  if (x < mi.rcWork.left) x = mi.rcWork.left;
  if (y + h > mi.rcWork.bottom) y = caret.top - h - Scale(2, effective_dpi);
  if (y < mi.rcWork.top) y = mi.rcWork.top;
  ImeLog("cand layout lines=%zu page=%d dpi=%u w=%d h=%d at (%d,%d)",
         lines_.size(), page_no_, effective_dpi, w, h, x, y);
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
  version_ = snap.version;
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
  FillRect(dc, &rc, reinterpret_cast<HBRUSH>(
                        GetClassLongPtrW(wnd_, GCLP_HBRBACKGROUND)));
  SetBkMode(dc, TRANSPARENT);
  SelectObject(dc, font_);
  int y = pad_y_;
  const HBRUSH highlight = CreateSolidBrush(RGB(0x2B, 0x6C, 0xF0));
  for (size_t i = 0; i < lines_.size(); ++i) {
    if (static_cast<int>(i) == highlighted_) {
      RECT row = {0, y, rc.right, y + row_h_};
      FillRect(dc, &row, highlight);
      SetTextColor(dc, RGB(0xFF, 0xFF, 0xFF));
    } else {
      SetTextColor(dc, RGB(0x20, 0x20, 0x20));
    }
    TextOutW(dc, pad_x_, y + Scale(4, GetDpiForWindow(wnd_)),
             labels_[i].c_str(), static_cast<int>(labels_[i].size()));
    TextOutW(dc, pad_x_ + label_w_, y + Scale(4, GetDpiForWindow(wnd_)),
             lines_[i].c_str(), static_cast<int>(lines_[i].size()));
    y += row_h_;
  }
  DeleteObject(highlight);
}

void CandidateWindow::OnClick(int y) {
  if (lines_.empty()) return;
  const int row = (y - pad_y_) / (row_h_ > 0 ? row_h_ : 1);
  if (row < 0 || row >= static_cast<int>(lines_.size())) return;
  diag::Event("candidate", "click");
  if (handler_) handler_(row, version_);
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
    case WM_LBUTTONUP:
      if (self) self->OnClick(static_cast<short>(HIWORD(lparam)));
      return 0;
    case WM_MOUSEACTIVATE:
      return MA_NOACTIVATE;  // never steal editor focus
    case WM_DPICHANGED:
      InvalidateRect(wnd, nullptr, TRUE);
      return 0;
    default:
      return DefWindowProcW(wnd, msg, wparam, lparam);
  }
}
