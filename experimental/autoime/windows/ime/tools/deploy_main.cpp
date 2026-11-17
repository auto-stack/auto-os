// AutoIME deployment CLI (PLAN-070 T-04).
//
// Synchronously deploys the shipped Rime resources into AutoIME's private
// user dir, compiles them, validates the effective schema (with the last
// known-good rollback and the Luna fallback), measures the first-deploy time
// and the ready-state key-to-candidate latency, and prints one JSON line.
//
// Exit codes: 0 primary schema ready, 2 rime.dll unavailable (pass-through),
// 3 engine disabled, 5 Luna fallback ready, 6 bad usage.
#include "../deployment.h"
#include "../diag.h"
#include "../rime_engine.h"

#include <windows.h>

#include <algorithm>
#include <cstdarg>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <string>
#include <vector>

// The library calls ImeLog for structural diagnostics; this CLI writes them to
// stderr only (payload-free by contract).
void ImeLog(const char* fmt, ...) {
  va_list args;
  va_start(args, fmt);
  std::vfprintf(stderr, fmt, args);
  va_end(args);
  std::fputc('\n', stderr);
}

namespace {

std::string ExeDir() {
  char path[MAX_PATH]{};
  GetModuleFileNameA(nullptr, path, MAX_PATH);
  std::string dir(path);
  const size_t slash = dir.find_last_of("\\/");
  if (slash != std::string::npos) dir.resize(slash);
  return dir;
}

long long NowUs() {
  static LARGE_INTEGER freq{};
  if (!freq.QuadPart) QueryPerformanceFrequency(&freq);
  LARGE_INTEGER now{};
  QueryPerformanceCounter(&now);
  return freq.QuadPart ? now.QuadPart * 1000000 / freq.QuadPart : 0;
}

struct Latency {
  int samples = 0;
  long long median_us = 0;
  long long p95_us = 0;
  long long max_us = 0;
};

// Time from the first key of "nihao" to a non-empty candidate page, on a
// warmed engine (first two iterations are warm-up only).
Latency MeasureLatency(RimeEngine* engine, uintptr_t session, int samples) {
  Latency result;
  if (!session || samples <= 0) return result;
  std::vector<long long> times;
  for (int i = 0; i < samples + 2; ++i) {
    engine->ClearComposition(session);
    const long long start = NowUs();
    const char keys[] = "nihao";
    for (const char* p = keys; *p; ++p) {
      engine->ProcessKey(session, *p, 0);
    }
    RimeEngine::Snapshot snap;
    bool have = false;
    if (engine->GetContext(session, &snap) && !snap.candidates.empty())
      have = true;
    const long long elapsed = NowUs() - start;
    if (have && i >= 2) times.push_back(elapsed);
    engine->ClearComposition(session);
  }
  engine->ClearComposition(session);
  if (times.empty()) return result;
  std::sort(times.begin(), times.end());
  result.samples = static_cast<int>(times.size());
  result.median_us = times[times.size() / 2];
  const size_t p95_index =
      static_cast<size_t>(0.95 * (times.size() - 1));
  result.p95_us = times[p95_index];
  result.max_us = times.back();
  return result;
}

std::string JsonEscape(const std::string& s) {
  std::string out;
  for (char c : s) {
    if (c == '"' || c == '\\') {
      out.push_back('\\');
      out.push_back(c);
    } else if (c == '\n' || c == '\r' || c == '\t') {
      out.push_back(' ');
    } else {
      out.push_back(c);
    }
  }
  return out;
}

}  // namespace

int main(int argc, char** argv) {
  std::string resources_dir;
  std::string user_dir;
  int latency_samples = 0;
  bool cleanup_legacy = false;
  for (int i = 1; i < argc; ++i) {
    const std::string arg = argv[i];
    auto value = [&]() -> const char* {
      if (i + 1 >= argc) return nullptr;
      return argv[++i];
    };
    if (arg == "--resources-dir") {
      const char* v = value();
      if (!v) return 6;
      resources_dir = v;
    } else if (arg == "--user-data-dir") {
      const char* v = value();
      if (!v) return 6;
      user_dir = v;
    } else if (arg == "--latency") {
      const char* v = value();
      if (!v) return 6;
      latency_samples = std::atoi(v);
    } else if (arg == "--cleanup-legacy-log") {
      cleanup_legacy = true;
    } else {
      std::fprintf(stderr, "unknown argument: %s\n", arg.c_str());
      return 6;
    }
  }

  if (cleanup_legacy) {
    std::string path;
    const bool removed = deployment::CleanupLegacyLog(&path);
    std::printf("{\"legacy_log\": \"%s\", \"removed\": %d}\n",
                JsonEscape(path).c_str(), removed ? 1 : 0);
    return 0;
  }

  const std::string module_dir = ExeDir();
  if (resources_dir.empty())
    resources_dir = deployment::ResolveResourcesDir(module_dir);
  if (user_dir.empty()) user_dir = deployment::DefaultUserDir();

  RimeEngine engine;
  engine.SetModuleDir(module_dir);
  engine.SetPaths(resources_dir, user_dir);

  const long long deploy_start = NowUs();
  std::string error;
  const bool ok = engine.EnsureReady(&error);
  const long long deploy_ms = (NowUs() - deploy_start) / 1000;

  Latency latency;
  if (ok) {
    const uintptr_t session = engine.CreateSession();
    if (session) {
      latency = MeasureLatency(&engine, session, latency_samples);
      engine.DestroySession(session);
    }
  }

  const bool fallback = engine.fallback();
  std::printf(
      "{\"deploy_ms\": %lld, \"ready\": %d, \"fallback\": %d, "
      "\"disabled\": %d, \"schema\": \"%s\", \"resources_dir\": \"%s\", "
      "\"user_data_dir\": \"%s\", \"resources_version\": \"%s\", "
      "\"detail\": \"%s\", \"latency_n\": %d, \"median_us\": %lld, "
      "\"p95_us\": %lld, \"max_us\": %lld, \"error\": \"%s\"}\n",
      deploy_ms, ok ? 1 : 0, fallback ? 1 : 0, engine.disabled() ? 1 : 0,
      JsonEscape(engine.schema()).c_str(), JsonEscape(resources_dir).c_str(),
      JsonEscape(user_dir).c_str(),
      JsonEscape(engine.deployment_version()).c_str(),
      JsonEscape(engine.deployment_detail()).c_str(), latency.samples,
      latency.median_us, latency.p95_us, latency.max_us,
      JsonEscape(error).c_str());
  const int code = !ok ? (engine.disabled() ? 3 : 2) : (fallback ? 5 : 0);
  engine.Shutdown();
  std::fflush(stdout);
  // rime.dll keeps background state whose CRT teardown is unreliable; the
  // engine has been finalized above, so exit directly.
  std::_Exit(code);
}
