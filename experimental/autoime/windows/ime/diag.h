// AutoIME - payload-free diagnostics (PLAN-070 T-01).
//
// Default diagnostics may record ONLY: version/identity, status codes,
// failure stage, aggregate counts and timings. They must never contain a key
// code sequence, pre-edit text, selection text or committed text.
//
// The API enforces that by construction: stage and code are fixed tokens and
// the only numeric payload is a counter/duration. There is deliberately no
// printf-style entry point here; structural lines (paths, rectangles, HRESULT
// values) stay in ImeLog and are audited separately.
#ifndef AUTOIME_DIAG_H_
#define AUTOIME_DIAG_H_

#include <string>

namespace diag {

// Opens the diagnostic sink. An empty path selects "%TEMP%\autoime.log".
// Safe to call more than once; the first path wins until Shutdown().
void Init(const std::string& log_path = std::string());
void Shutdown();

// Emits one payload-free event: stage token + result token.
void Event(const char* stage, const char* code);
// Same, with one integer counter/duration (milliseconds, attempts, counts).
void Count(const char* stage, const char* code, long long value);
// Records a failed stage with its HRESULT/error code (no other detail).
void Failure(const char* stage, long hr);

const std::string& log_path();

}  // namespace diag

#endif  // AUTOIME_DIAG_H_
