// AutoIME automated fixture (PLAN-070 T-01).
//
// Replays the TSF callback shapes that PLAN-069 left unverified, against the
// real KeyRouter used in production:
//   * Test -> Test -> Down        (repeated Test callback)
//   * Down only                   (hosts that skip Test)
//   * Test -> Down ... repeated   (keyboard auto-repeat)
//   * Test, focus change, Down    (late callback from the previous focus)
// plus modifier ownership (Ctrl/Alt/Win/AltGr), Shift-click mode switching,
// Shift+letter case, sensitive scopes, idle vs composing keys and the
// payload-free diagnostics contract.
//
// Exit code 0 = all cases pass, 5 = at least one case failed.
#include "../diag.h"
#include "../edit_model.h"
#include "../key_router.h"

#include <windows.h>

#include <cctype>
#include <cstdio>
#include <cstring>
#include <string>

namespace {

int g_checks = 0;
int g_failures = 0;

#define CHECK(cond)                                                        \
  do {                                                                     \
    ++g_checks;                                                            \
    if (!(cond)) {                                                         \
      ++g_failures;                                                        \
      std::printf("FAIL %s:%d: %s\n", __FILE__, __LINE__, #cond);          \
    }                                                                      \
  } while (0)

struct FakeTarget final : IKeyTarget {
  bool composing = false;
  bool ascii = false;
  bool sensitive = false;
  bool ready = true;
  bool accept = true;
  int ops = 0;
  int toggles = 0;
  int cancels = 0;
  int last_keysym = -1;
  int last_mask = -1;

  bool Composing() override { return composing; }
  bool AsciiMode() override { return ascii; }
  bool EngineReady() override { return ready; }
  bool Sensitive() override { return sensitive; }
  bool ProcessKey(int keysym, int mask) override {
    ++ops;
    last_keysym = keysym;
    last_mask = mask;
    return accept;
  }
  void ToggleMode() override {
    ++toggles;
    ascii = !ascii;
  }
  void CancelComposition() override {
    ++cancels;
    composing = false;
  }
};

KeyEvent Make(WPARAM vk, bool shift = false, bool ctrl = false, bool alt = false,
              bool win = false, bool repeat = false) {
  KeyEvent key;
  key.vk = vk;
  key.mods.shift = shift;
  key.mods.ctrl = ctrl;
  key.mods.alt = alt;
  key.mods.win = win;
  key.repeat = repeat;
  return key;
}

void CaseRepeatedTestProcessesOnce() {
  FakeTarget t;
  KeyRouter r;
  r.SetTarget(&t);
  const KeyEvent n = Make('N');
  const RouteDecision first = r.OnTestKeyDown(n, 1);
  const RouteDecision second = r.OnTestKeyDown(n, 1);
  CHECK(first.eat && second.eat);
  CHECK(t.ops == 0);  // Test must never advance the engine
  const RouteDecision down = r.OnKeyDown(n, 1);
  CHECK(down.eat);
  CHECK(t.ops == 1);
  CHECK(t.last_keysym == 'n');
}

void CaseDownOnlyProcessesOnce() {
  FakeTarget t;
  KeyRouter r;
  r.SetTarget(&t);
  const RouteDecision d = r.OnKeyDown(Make('N'), 1);
  CHECK(d.eat && t.ops == 1 && t.last_keysym == 'n');
}

void CaseAutoRepeatProcessesEachPress() {
  FakeTarget t;
  KeyRouter r;
  r.SetTarget(&t);
  const KeyEvent first = Make('N');
  const KeyEvent repeat = Make('N', false, false, false, false, true);
  r.OnTestKeyDown(first, 1);
  r.OnKeyDown(first, 1);
  r.OnTestKeyDown(repeat, 1);
  r.OnKeyDown(repeat, 1);
  CHECK(t.ops == 2);
  // Down-only host auto-repeat.
  FakeTarget t2;
  KeyRouter r2;
  r2.SetTarget(&t2);
  r2.OnKeyDown(first, 1);
  r2.OnKeyDown(repeat, 1);
  CHECK(t2.ops == 2);
}

void CaseStaleFocusCallbackDropped() {
  FakeTarget t;
  KeyRouter r;
  r.SetTarget(&t);
  const KeyEvent n = Make('N');
  CHECK(r.OnTestKeyDown(n, 1).eat);
  r.OnFocus(2, true);
  const RouteDecision late = r.OnKeyDown(n, 1);  // still tagged with old focus
  CHECK(!late.eat);
  CHECK(t.ops == 0);
  const RouteDecision fresh = r.OnKeyDown(n, 2);
  CHECK(fresh.eat);
  CHECK(t.ops == 1);
  // A Test from the old generation is dropped too.
  CHECK(!r.OnTestKeyDown(n, 1).eat);
}

void CaseShortcutsStayWithHost() {
  FakeTarget t;
  KeyRouter r;
  r.SetTarget(&t);
  const KeyEvent ctrl_s = Make('S', false, true);
  const KeyEvent alt_tab = Make(VK_TAB, false, false, true);
  const KeyEvent win = Make('E', false, false, false, true);
  const KeyEvent altgr = Make('E', false, true, true);  // Ctrl+Alt = AltGr
  CHECK(!r.OnKeyDown(ctrl_s, 1).eat);
  CHECK(!r.OnKeyDown(alt_tab, 1).eat);
  CHECK(!r.OnKeyDown(win, 1).eat);
  CHECK(!r.OnKeyDown(altgr, 1).eat);
  CHECK(t.ops == 0);
}

void CaseShiftClickTogglesMode() {
  FakeTarget t;
  KeyRouter r;
  r.SetTarget(&t);
  r.OnKeyDown(Make(VK_SHIFT, true), 1);
  r.OnKeyUp(Make(VK_SHIFT, true), 1);
  CHECK(t.toggles == 1);
  CHECK(t.ascii);
  // In English mode Shift+letter is the host's own capital letter.
  const KeyEvent shift_letter = Make('A', true);
  r.OnKeyDown(Make(VK_SHIFT, true), 1);
  r.OnKeyDown(shift_letter, 1);
  r.OnKeyUp(Make(VK_SHIFT, true), 1);
  CHECK(t.toggles == 1);
  CHECK(t.ops == 0);
  // Back in Chinese mode Shift+letter keeps its case for the engine and does
  // not toggle the mode.
  t.ascii = false;
  r.OnKeyDown(Make(VK_SHIFT, true), 1);
  r.OnKeyDown(shift_letter, 1);
  r.OnKeyUp(Make(VK_SHIFT, true), 1);
  CHECK(t.toggles == 1);
  CHECK(t.ops == 1);
  CHECK(t.last_keysym == 'A');
  CHECK(t.last_mask == 1);
  // Ctrl+Shift is a chord, not a click.
  r.OnKeyDown(Make(VK_SHIFT, true, true), 1);
  r.OnKeyUp(Make(VK_SHIFT, true, true), 1);
  CHECK(t.toggles == 1);
}

void CaseShiftClickCancelsComposition() {
  FakeTarget t;
  t.composing = true;
  KeyRouter r;
  r.SetTarget(&t);
  r.OnKeyDown(Make(VK_SHIFT, true), 1);
  r.OnKeyUp(Make(VK_SHIFT, true), 1);
  CHECK(t.cancels == 1);
  CHECK(t.toggles == 1);
}

void CaseSensitiveAndUnreadyPassThrough() {
  FakeTarget t;
  KeyRouter r;
  r.SetTarget(&t);
  t.sensitive = true;
  CHECK(!r.OnKeyDown(Make('N'), 1).eat);
  CHECK(!r.OnTestKeyDown(Make('N'), 1).eat);
  CHECK(t.ops == 0);
  t.sensitive = false;
  t.ready = false;
  CHECK(!r.OnKeyDown(Make('N'), 1).eat);
  CHECK(t.ops == 0);
}

void CaseAsciiModeTypesDirectly() {
  FakeTarget t;
  KeyRouter r;
  r.SetTarget(&t);
  t.ascii = true;
  CHECK(!r.OnKeyDown(Make('N'), 1).eat);
  CHECK(t.ops == 0);
  t.ascii = false;
  CHECK(r.OnKeyDown(Make('N'), 1).eat);
  CHECK(t.ops == 1);
}

void CaseIdleKeysStayWithHost() {
  FakeTarget t;
  KeyRouter r;
  r.SetTarget(&t);
  const WPARAM idle[] = {VK_SPACE, '1', VK_ESCAPE, VK_RETURN, VK_BACK,
                         VK_LEFT,  VK_PRIOR, VK_NEXT, VK_TAB};
  for (WPARAM vk : idle) CHECK(!r.OnKeyDown(Make(vk), 1).eat);
  CHECK(!r.OnKeyDown(Make(VK_F1), 1).eat);
  CHECK(t.ops == 0);
}

void CaseComposingKeysAreConsumed() {
  FakeTarget t;
  t.composing = true;
  KeyRouter r;
  r.SetTarget(&t);
  const WPARAM composing[] = {VK_SPACE, '1',  VK_ESCAPE, VK_RETURN,
                              VK_BACK,  VK_LEFT, VK_PRIOR,  VK_NEXT,
                              VK_TAB,   VK_OEM_COMMA};
  for (WPARAM vk : composing) CHECK(r.OnKeyDown(Make(vk), 1).eat);
  CHECK(t.ops == 10);
}

void CasePunctuationFollowsMode() {
  FakeTarget t;
  KeyRouter r;
  r.SetTarget(&t);
  CHECK(r.OnKeyDown(Make(VK_OEM_COMMA), 1).eat);  // Chinese punctuation
  t.ascii = true;
  CHECK(!r.OnKeyDown(Make(VK_OEM_COMMA), 1).eat);  // English punctuation
}

void CaseGeneratedKeysymMapping() {
  CHECK(KeysymForVk('A', false) == 'a');
  CHECK(KeysymForVk('A', true) == 'A');
  CHECK(KeysymForVk(VK_SPACE, false) == 0x20);
  CHECK(KeysymForVk(VK_ESCAPE, false) == 0xff1b);
  CHECK(KeysymForVk(VK_F1, false) == 0);
  CHECK(RimeModifierMask(Make('A', true).mods) == 1);
  CHECK(RimeModifierMask(Make('A', false, true, true).mods) == 12);
}

// The diagnostics log produced while driving a full synthetic key session
// must contain no key codes, pre-edit, selection or committed text.
void CaseDiagnosticsArePayloadFree() {
  char temp[MAX_PATH]{};
  GetTempPathA(MAX_PATH, temp);
  char path[MAX_PATH]{};
  std::snprintf(path, sizeof(path), "%sautoime-fixture-%lu.log", temp,
                static_cast<unsigned long>(GetCurrentProcessId()));
  DeleteFileA(path);
  diag::Init(path);
  {
    FakeTarget t;
    KeyRouter r;
    r.SetTarget(&t);
    const char* payloads[] = {"nihao", "mima", "你好", "密码", "payload"};
    for (const char* p : payloads) {
      KeyEvent key = Make(static_cast<WPARAM>(
          toupper(static_cast<unsigned char>(p[0]))));
      r.OnTestKeyDown(key, 1);
      r.OnKeyDown(key, 1);
    }
    r.OnKeyDown(Make(VK_SHIFT, true), 1);
    r.OnKeyUp(Make(VK_SHIFT, true), 1);
    r.OnKeyDown(Make(VK_ESCAPE), 1);
    r.OnKeyDown(Make('1'), 1);
    r.OnFocus(2, false);
    r.OnKeyDown(Make('N'), 1);
  }
  FILE* f = std::fopen(path, "rb");
  CHECK(f != nullptr);
  std::string content;
  if (f) {
    char buf[4096];
    size_t n = 0;
    while ((n = std::fread(buf, 1, sizeof(buf), f)) > 0) content.append(buf, n);
    std::fclose(f);
  }
  for (const char* payload : {"nihao", "mima", "\xE4\xBD\xA0\xE5\xA5\xBD",
                              "\xE5\xAF\x86\xE7\xA0\x81", "payload", "vk="})
    CHECK(content.find(payload) == std::string::npos);
  CHECK(content.find("stage=key") != std::string::npos);
  // Closed vocabulary: every emitted code token is one of the known ones.
  const char* allowed[] = {"test_eat",      "test_pass", "op",
                           "op_declined",   "pass",      "stale_dropped",
                           "repeat",        "latency_us", "shift_click_toggle",
                           "in",            "out",       "reset"};
  size_t pos = 0;
  int lines = 0;
  while (pos < content.size()) {
    const size_t end = content.find('\n', pos);
    const std::string line = content.substr(
        pos, end == std::string::npos ? std::string::npos : end - pos);
    pos = end == std::string::npos ? content.size() : end + 1;
    ++lines;
    CHECK(line.size() < 200);
    const size_t code_at = line.find("code=");
    CHECK(code_at != std::string::npos);
    if (code_at == std::string::npos) continue;
    std::string code = line.substr(code_at + 5);
    const size_t space = code.find(' ');
    if (space != std::string::npos) code = code.substr(0, space);
    bool known = false;
    for (const char* a : allowed) known = known || code == a;
    CHECK(known);
  }
  CHECK(lines > 0);
  diag::Shutdown();
}

// T-02: the commit/pre-edit combinations decide whether the prefix is
// written once, a new composition opens after it, or the original text is
// restored.
void CaseCompositionApplyPlan() {
  using autoime::EditAction;
  using autoime::PlanCompositionApply;
  CHECK(PlanCompositionApply(true, false, true) == EditAction::kCommit);
  CHECK(PlanCompositionApply(true, false, false) ==
        EditAction::kCommitThenCompose);
  CHECK(PlanCompositionApply(false, true, false) == EditAction::kCompose);
  CHECK(PlanCompositionApply(false, true, true) == EditAction::kRestore);
  // An empty commit string never writes anything.
  CHECK(PlanCompositionApply(true, true, true) == EditAction::kRestore);
}

// T-02: emoji and the surrounding text survive UTF-8 -> UTF-16 conversion.
void CaseEmojiConversionKeepsPrefixAndSuffix() {
  // U+7532 U+1F600 U+4E59 = 甲😀乙
  const std::string utf8 = "\xE7\x94\xB2\xF0\x9F\x98\x80\xE4\xB9\x99";
  const std::wstring wide = autoime::Utf8ToUtf16(utf8);
  CHECK(wide.size() == 4);  // 甲 + surrogate pair + 乙
  if (wide.size() == 4) {
    CHECK(wide[0] == 0x7532);
    CHECK(wide[1] == 0xD83D);
    CHECK(wide[2] == 0xDE00);
    CHECK(wide[3] == 0x4E59);
  }
  CHECK(autoime::Utf8ToUtf16("AutoOS").size() == 6);
  CHECK(autoime::Utf8ToUtf16("").empty());
}

}  // namespace

int main() {
  SetConsoleOutputCP(CP_UTF8);
  CaseRepeatedTestProcessesOnce();
  CaseDownOnlyProcessesOnce();
  CaseAutoRepeatProcessesEachPress();
  CaseStaleFocusCallbackDropped();
  CaseShortcutsStayWithHost();
  CaseShiftClickTogglesMode();
  CaseShiftClickCancelsComposition();
  CaseSensitiveAndUnreadyPassThrough();
  CaseAsciiModeTypesDirectly();
  CaseIdleKeysStayWithHost();
  CaseComposingKeysAreConsumed();
  CasePunctuationFollowsMode();
  CaseGeneratedKeysymMapping();
  CaseCompositionApplyPlan();
  CaseEmojiConversionKeepsPrefixAndSuffix();
  CaseDiagnosticsArePayloadFree();

  std::printf("SUMMARY {\"fixture\": \"key_router\", \"checks\": %d, "
              "\"failures\": %d}\n",
              g_checks, g_failures);
  return g_failures == 0 ? 0 : 5;
}
