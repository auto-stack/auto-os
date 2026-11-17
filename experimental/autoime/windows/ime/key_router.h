// AutoIME - TSF key routing state machine (PLAN-070 T-01).
//
// Windows hosts drive a TSF key sink in different shapes:
//   * Test -> Down                     (normal)
//   * Test -> Test -> Down             (some hosts repeat the Test callback)
//   * Down only                        (hosts that skip the Test callback)
//   * Test -> Down, Test -> Down, ...  (keyboard auto-repeat)
//
// The router guarantees that one physical key press produces at most ONE
// engine operation, that a late callback from an old focus generation is
// dropped, and that Test never mutates the document (the engine is only
// advanced on the matching KeyDown).
//
// The router is deliberately independent of TSF so the callback sequences can
// be replayed by an automated fixture, and it only emits payload-free
// diagnostics (stage/code/counts, never key codes or text).
#ifndef AUTOIME_KEY_ROUTER_H_
#define AUTOIME_KEY_ROUTER_H_

#include <cstdint>
#include <windows.h>

struct KeyModifiers {
  bool shift = false;
  bool ctrl = false;
  bool alt = false;
  bool win = false;
};

struct KeyEvent {
  WPARAM vk = 0;
  KeyModifiers mods;
  // WM_KEYDOWN lParam bit 30 (previous key state down): true for auto-repeat.
  bool repeat = false;
};

// Closed vocabulary: diagnostics never carry a VK code, a keysym or text.
enum class RouteReason {
  kPassUnmapped,
  kPassIdle,
  kPassShortcut,
  kPassSensitive,
  kPassAsciiMode,
  kPassShift,
  kPassStaleContext,
  kEatLetter,
  kEatComposingKey,
  kEatPunctuation,
};

struct RouteDecision {
  bool eat = false;
  RouteReason reason = RouteReason::kPassUnmapped;
  int keysym = 0;
  int rime_modifiers = 0;
};

// Everything the router needs from the text service. Implemented by
// CTextService in production and by a counting fake in the fixture.
class IKeyTarget {
 public:
  virtual ~IKeyTarget() = default;
  virtual bool Composing() = 0;
  virtual bool AsciiMode() = 0;
  // False when the engine is missing/deploying: every key passes through.
  virtual bool EngineReady() = 0;
  // Password fields / host-disabled contexts hand every key back to the host.
  virtual bool Sensitive() = 0;
  // Advances the engine by exactly one operation. Returns whether the engine
  // consumed the key.
  virtual bool ProcessKey(int keysym, int rime_modifiers) = 0;
  // Single Shift-click switches Chinese/English (ascii_mode).
  virtual void ToggleMode() = 0;
  // Discards the current composition (used when the mode switch cancels it).
  virtual void CancelComposition() = 0;
};

// Maps a Windows VK to an X11-style Rime keysym. Letters honour Shift so
// mixed Chinese/English input keeps its case. Returns 0 when unmapped.
int KeysymForVk(WPARAM vk, bool shift);

// Rime modifier mask built from a modifier snapshot.
int RimeModifierMask(const KeyModifiers& mods);

class KeyRouter {
 public:
  void SetTarget(IKeyTarget* target) { target_ = target; }
  IKeyTarget* target() const { return target_; }

  // Focus/context handoff. `generation` must strictly increase per focus
  // change; callbacks carrying an older generation are dropped.
  void OnFocus(uint64_t generation, bool foreground);

  // Returns the decision for this callback. OnKeyDown performs the engine
  // operation exactly once; OnTestKeyDown never does.
  RouteDecision OnTestKeyDown(const KeyEvent& key, uint64_t generation);
  RouteDecision OnKeyDown(const KeyEvent& key, uint64_t generation);
  void OnKeyUp(const KeyEvent& key, uint64_t generation);

  void Reset();

  uint64_t generation() const { return generation_; }
  bool shift_down() const { return shift_down_; }

 private:
  RouteDecision Decide(const KeyEvent& key, uint64_t generation) const;
  bool MatchesPending(const KeyEvent& key, uint64_t generation) const;

  IKeyTarget* target_ = nullptr;
  uint64_t generation_ = 1;
  struct PendingTest {
    bool valid = false;
    KeyEvent key;
    uint64_t generation = 0;
    RouteDecision decision;
  } pending_;
  bool shift_down_ = false;
  bool shift_used_ = false;
};

#endif  // AUTOIME_KEY_ROUTER_H_
