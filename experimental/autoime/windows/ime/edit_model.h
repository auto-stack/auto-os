// AutoIME - composition/edit planning helpers (PLAN-070 T-02).
//
// Keeping the apply plan pure makes the tricky combinations testable without
// a TSF host: a commit with leftover pre-edit must write the prefix exactly
// once before a new composition opens, a bare cancel must restore, and UTF-8
// to UTF-16 conversion must preserve surrogate pairs (emoji) and the
// surrounding text.
#ifndef AUTOIME_EDIT_MODEL_H_
#define AUTOIME_EDIT_MODEL_H_

#include <string>

namespace autoime {

// UTF-8 -> UTF-16. Surrogate pairs are emitted as two WCHARs; the length is
// the number of WCHARs, never bytes.
std::wstring Utf8ToUtf16(const std::string& utf8);

enum class EditAction {
  kNone,              // nothing to apply
  kCommit,            // write the commit once, composition ends
  kCommitThenCompose, // write the commit once, then open a new composition
  kCompose,           // update/create the composition only
  kRestore,           // cancel: revert to the original text/selection
};

// Decides what one engine result means for the document.
EditAction PlanCompositionApply(bool has_commit, bool commit_empty,
                                bool preedit_empty);

}  // namespace autoime

#endif  // AUTOIME_EDIT_MODEL_H_
