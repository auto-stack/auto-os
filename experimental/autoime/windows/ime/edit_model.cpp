// AutoIME - composition/edit planning helpers (PLAN-070 T-02).
#include "edit_model.h"

#include <windows.h>

namespace autoime {

std::wstring Utf8ToUtf16(const std::string& utf8) {
  if (utf8.empty()) return std::wstring();
  const int n = MultiByteToWideChar(CP_UTF8, 0, utf8.c_str(),
                                    static_cast<int>(utf8.size()), nullptr, 0);
  if (n <= 0) return std::wstring();
  std::wstring out(static_cast<size_t>(n), L'\0');
  MultiByteToWideChar(CP_UTF8, 0, utf8.c_str(), static_cast<int>(utf8.size()),
                      &out[0], n);
  return out;
}

EditAction PlanCompositionApply(bool has_commit, bool commit_empty,
                                bool preedit_empty) {
  const bool commit = has_commit && !commit_empty;
  if (commit && !preedit_empty) return EditAction::kCommitThenCompose;
  if (commit) return EditAction::kCommit;
  if (!preedit_empty) return EditAction::kCompose;
  return EditAction::kRestore;
}

}  // namespace autoime
