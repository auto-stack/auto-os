// AutoIME - payload-free diagnostics implementation (PLAN-070 T-01).
#include "diag.h"

#include <windows.h>

#include <cstdio>
#include <cstring>
#include <mutex>

namespace diag {
namespace {

std::mutex g_mutex;
FILE* g_file = nullptr;
std::string g_path;

std::string DefaultPath() {
  char temp[MAX_PATH]{};
  if (GetTempPathA(MAX_PATH, temp) == 0) return std::string("autoime.log");
  std::string path(temp);
  if (!path.empty() && path.back() != '\\' && path.back() != '/')
    path += "\\";
  return path + "autoime.log";
}

void WriteLine(const char* line) {
  std::lock_guard<std::mutex> lock(g_mutex);
  if (!g_file) return;
  std::fputs(line, g_file);
  std::fputc('\n', g_file);
  std::fflush(g_file);
}

void Emit(const char* stage, const char* code, const char* extra) {
  if (!g_file || !stage || !code) return;
  SYSTEMTIME st{};
  GetLocalTime(&st);
  char line[256];
  std::snprintf(line, sizeof(line),
                "%04u-%02u-%02u %02u:%02u:%02u.%03u pid=%lu stage=%s "
                "code=%s%s",
                st.wYear, st.wMonth, st.wDay, st.wHour, st.wMinute, st.wSecond,
                st.wMilliseconds, static_cast<unsigned long>(GetCurrentProcessId()),
                stage, code, extra ? extra : "");
  WriteLine(line);
}

}  // namespace

void Init(const std::string& log_path) {
  std::lock_guard<std::mutex> lock(g_mutex);
  if (g_file) return;
  g_path = log_path.empty() ? DefaultPath() : log_path;
  g_file = std::fopen(g_path.c_str(), "ab");
}

void Shutdown() {
  std::lock_guard<std::mutex> lock(g_mutex);
  if (g_file) {
    std::fclose(g_file);
    g_file = nullptr;
  }
}

void Event(const char* stage, const char* code) { Emit(stage, code, nullptr); }

void Count(const char* stage, const char* code, long long value) {
  char extra[64];
  std::snprintf(extra, sizeof(extra), " n=%lld", value);
  Emit(stage, code, extra);
}

void Failure(const char* stage, long hr) {
  char extra[64];
  std::snprintf(extra, sizeof(extra), " hr=0x%08lx",
                static_cast<unsigned long>(hr));
  Emit(stage, "fail", extra);
}

const std::string& log_path() { return g_path; }

}  // namespace diag
