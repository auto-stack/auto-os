// AutoIME - key routing state machine implementation (PLAN-070 T-01).
#include "key_router.h"

#include "diag.h"

namespace {

bool SameModifiers(const KeyModifiers& a, const KeyModifiers& b) {
  return a.shift == b.shift && a.ctrl == b.ctrl && a.alt == b.alt &&
         a.win == b.win;
}

bool IsLetter(WPARAM vk) { return vk >= 'A' && vk <= 'Z'; }
bool IsDigit(WPARAM vk) { return vk >= '0' && vk <= '9'; }

bool IsPunctuation(WPARAM vk) {
  switch (vk) {
    case VK_OEM_1:
    case VK_OEM_PLUS:
    case VK_OEM_COMMA:
    case VK_OEM_MINUS:
    case VK_OEM_PERIOD:
    case VK_OEM_2:
    case VK_OEM_3:
    case VK_OEM_4:
    case VK_OEM_5:
    case VK_OEM_6:
    case VK_OEM_7:
      return true;
    default:
      return false;
  }
}

// Keys a composition consumes: edit, navigate and commit while composing.
bool IsComposingKey(WPARAM vk) {
  switch (vk) {
    case VK_SPACE:
    case VK_BACK:
    case VK_ESCAPE:
    case VK_RETURN:
    case VK_DELETE:
    case VK_HOME:
    case VK_END:
    case VK_LEFT:
    case VK_UP:
    case VK_RIGHT:
    case VK_DOWN:
    case VK_PRIOR:
    case VK_NEXT:
    case VK_TAB:
      return true;
    default:
      return false;
  }
}

}  // namespace

int RimeModifierMask(const KeyModifiers& mods) {
  // librime RimeModifier bits: shift=1, lock=2, control=4, alt=8, super=16.
  int mask = 0;
  if (mods.shift) mask |= 1;
  if (mods.ctrl) mask |= 4;
  if (mods.alt) mask |= 8;
  if (mods.win) mask |= 16;
  return mask;
}

int KeysymForVk(WPARAM vk, bool shift) {
  const DWORD v = static_cast<DWORD>(vk);
  if (v >= 'A' && v <= 'Z')
    return shift ? static_cast<int>(v) : static_cast<int>(v + 0x20);
  switch (v) {
    case VK_SPACE: return 0x0020;
    case VK_BACK: return 0xff08;
    case VK_ESCAPE: return 0xff1b;
    case VK_RETURN: return 0xff0d;
    case VK_TAB: return 0xff09;
    case VK_DELETE: return 0xffff;
    case VK_HOME: return 0xff50;
    case VK_LEFT: return 0xff51;
    case VK_UP: return 0xff52;
    case VK_RIGHT: return 0xff53;
    case VK_DOWN: return 0xff54;
    case VK_PRIOR: return 0xff55;
    case VK_NEXT: return 0xff56;
    case VK_END: return 0xff57;
    default: break;
  }
  if (v >= '0' && v <= '9') return static_cast<int>(v);
  switch (v) {
    case VK_OEM_1: return ';';
    case VK_OEM_PLUS: return '=';
    case VK_OEM_COMMA: return ',';
    case VK_OEM_MINUS: return '-';
    case VK_OEM_PERIOD: return '.';
    case VK_OEM_2: return '/';
    case VK_OEM_3: return '`';
    case VK_OEM_4: return '[';
    case VK_OEM_5: return '\\';
    case VK_OEM_6: return ']';
    case VK_OEM_7: return '\'';
    default: return 0;
  }
}

RouteDecision KeyRouter::Decide(const KeyEvent& key,
                                uint64_t generation) const {
  RouteDecision d;
  if (!target_) return d;
  if (generation != generation_) {
    d.reason = RouteReason::kPassStaleContext;
    return d;
  }
  // Without an engine the IME is transparent: the host keeps every key.
  if (!target_->EngineReady()) {
    d.reason = RouteReason::kPassIdle;
    return d;
  }
  if (target_->Sensitive()) {
    d.reason = RouteReason::kPassSensitive;
    return d;
  }
  // Host shortcuts keep ownership; this is also what preserves AltGr input
  // (Windows delivers those characters through the host's WM_CHAR path).
  if (key.mods.ctrl || key.mods.alt || key.mods.win) {
    d.reason = RouteReason::kPassShortcut;
    return d;
  }
  if (key.vk == VK_SHIFT) {
    d.reason = RouteReason::kPassShift;
    return d;
  }
  const int keysym = KeysymForVk(key.vk, key.mods.shift);
  if (keysym == 0) {
    d.reason = RouteReason::kPassUnmapped;
    return d;
  }
  const bool composing = target_->Composing();
  const bool ascii = target_->AsciiMode();
  d.keysym = keysym;
  d.rime_modifiers = RimeModifierMask(key.mods);

  if (IsLetter(key.vk)) {
    // English mode types directly; Chinese mode starts/extends a composition.
    if (ascii && !composing) {
      d.reason = RouteReason::kPassAsciiMode;
      return d;
    }
    d.eat = true;
    d.reason = RouteReason::kEatLetter;
    return d;
  }
  if (composing) {
    if (IsDigit(key.vk) || IsPunctuation(key.vk) || IsComposingKey(key.vk)) {
      d.eat = true;
      d.reason = RouteReason::kEatComposingKey;
      return d;
    }
    d.reason = RouteReason::kPassIdle;
    return d;
  }
  // Idle Chinese mode: punctuation goes through the schema's punctuator so
  // "," becomes "，". Everything else stays with the host.
  if (IsPunctuation(key.vk) && !ascii) {
    d.eat = true;
    d.reason = RouteReason::kEatPunctuation;
    return d;
  }
  d.reason = RouteReason::kPassIdle;
  return d;
}

bool KeyRouter::MatchesPending(const KeyEvent& key,
                               uint64_t generation) const {
  return pending_.valid && pending_.generation == generation &&
         pending_.key.vk == key.vk && pending_.key.repeat == key.repeat &&
         SameModifiers(pending_.key.mods, key.mods);
}

RouteDecision KeyRouter::OnTestKeyDown(const KeyEvent& key,
                                       uint64_t generation) {
  if (!target_) return RouteDecision{};
  if (generation != generation_) {
    pending_.valid = false;
    RouteDecision d;
    d.reason = RouteReason::kPassStaleContext;
    return d;
  }
  if (key.vk == VK_SHIFT) {
    RouteDecision d;
    d.reason = RouteReason::kPassShift;
    return d;
  }
  if (MatchesPending(key, generation)) return pending_.decision;
  const RouteDecision d = Decide(key, generation);
  pending_.valid = true;
  pending_.key = key;
  pending_.generation = generation;
  pending_.decision = d;
  diag::Event("key", d.eat ? "test_eat" : "test_pass");
  return d;
}

RouteDecision KeyRouter::OnKeyDown(const KeyEvent& key, uint64_t generation) {
  if (!target_) return RouteDecision{};
  if (generation != generation_) {
    // A late callback from a previous focus generation must not touch the
    // document, and it must not reuse a decision made for the old generation.
    pending_.valid = false;
    RouteDecision d;
    d.reason = RouteReason::kPassStaleContext;
    diag::Event("key", "stale_dropped");
    return d;
  }
  if (key.vk == VK_SHIFT) {
    shift_down_ = true;
    // Ctrl/Alt/Win + Shift is a shortcut chord, never a mode click.
    shift_used_ = key.mods.ctrl || key.mods.alt || key.mods.win;
    RouteDecision d;
    d.reason = RouteReason::kPassShift;
    return d;
  }
  if (shift_down_) shift_used_ = true;
  if (key.repeat) diag::Count("key", "repeat", 1);

  RouteDecision d;
  if (MatchesPending(key, generation)) {
    d = pending_.decision;
    pending_.valid = false;
  } else {
    pending_.valid = false;
    d = Decide(key, generation);
  }
  if (d.eat) {
    const bool consumed = target_->ProcessKey(d.keysym, d.rime_modifiers);
    diag::Event("key", consumed ? "op" : "op_declined");
  } else {
    diag::Event("key", "pass");
  }
  return d;
}

void KeyRouter::OnKeyUp(const KeyEvent& key, uint64_t generation) {
  pending_.valid = false;
  if (key.vk != VK_SHIFT) return;
  const bool click = shift_down_ && !shift_used_ && generation == generation_;
  shift_down_ = false;
  shift_used_ = false;
  if (!click || !target_) return;
  // An explicit mode switch cancels the live composition first.
  if (target_->Composing()) target_->CancelComposition();
  target_->ToggleMode();
  diag::Event("mode", "shift_click_toggle");
}

void KeyRouter::OnFocus(uint64_t generation, bool foreground) {
  if (generation > generation_) generation_ = generation;
  pending_.valid = false;
  shift_down_ = false;
  shift_used_ = false;
  diag::Event("focus", foreground ? "in" : "out");
}

void KeyRouter::Reset() {
  pending_.valid = false;
  shift_down_ = false;
  shift_used_ = false;
  diag::Event("router", "reset");
}
