// AutoIME TSF text service - DLL entry points (PLAN-069 T-02).
#include <initguid.h>
#include "ime.h"

#include <cstdarg>
#include <cstdio>
#include <cstring>
#include <new>

static LONG g_server_lock = 0;   // class factory LockServer count
static LONG g_objects = 0;       // live COM objects

HINSTANCE g_hInst = nullptr;

void AutoIMEAddRefObject() { InterlockedIncrement(&g_objects); }
void AutoIMEReleaseObject() { InterlockedDecrement(&g_objects); }
void AutoIMELockServer(BOOL fLock) {
  if (fLock)
    InterlockedIncrement(&g_server_lock);
  else
    InterlockedDecrement(&g_server_lock);
}

void ImeLog(const char* fmt, ...) {
  char path[MAX_PATH];
  if (GetTempPathA(MAX_PATH, path) == 0) return;
  if (strcat_s(path, "autoime-host.log") != 0) return;
  HANDLE file = CreateFileA(path, FILE_APPEND_DATA, FILE_SHARE_READ |
                            FILE_SHARE_WRITE, nullptr, OPEN_ALWAYS,
                            FILE_ATTRIBUTE_NORMAL, nullptr);
  if (file == INVALID_HANDLE_VALUE) return;
  SYSTEMTIME st;
  GetLocalTime(&st);
  char line[1024];
  int n = _snprintf_s(line, sizeof(line), _TRUNCATE,
                      "%04u-%02u-%02u %02u:%02u:%02u pid=%lu ",
                      st.wYear, st.wMonth, st.wDay, st.wHour, st.wMinute,
                      st.wSecond, GetCurrentProcessId());
  va_list args;
  va_start(args, fmt);
  // va_list requires the v-variant; passing it to _snprintf_s is UB.
  _vsnprintf_s(line + n, sizeof(line) - static_cast<size_t>(n), _TRUNCATE,
               fmt, args);
  va_end(args);
  strcat_s(line, "\r\n");
  DWORD written = 0;
  WriteFile(file, line, static_cast<DWORD>(strlen(line)), &written, nullptr);
  CloseHandle(file);
}

BOOL APIENTRY DllMain(HINSTANCE instance, DWORD reason, LPVOID) {
  if (reason == DLL_PROCESS_ATTACH) {
    g_hInst = instance;
    DisableThreadLibraryCalls(instance);
    char path[MAX_PATH]{};
    GetModuleFileNameA(instance, path, MAX_PATH);
    ImeLog("dll attach module=%s", path);
  } else if (reason == DLL_PROCESS_DETACH) {
    ImeLog("dll detach");
  }
  return TRUE;
}

STDAPI DllCanUnloadNow(void) {
  return (g_objects == 0 && g_server_lock == 0) ? S_OK : S_FALSE;
}

STDAPI DllGetClassObject(REFCLSID rclsid, REFIID riid, void** ppv) {
  *ppv = nullptr;
  if (!IsEqualCLSID(rclsid, CLSID_AutoIMETextService))
    return CLASS_E_CLASSNOTAVAILABLE;
  auto* factory = new (std::nothrow) CClassFactory();
  if (!factory) return E_OUTOFMEMORY;
  const HRESULT hr = factory->QueryInterface(riid, ppv);
  factory->Release();
  return hr;
}

STDAPI DllRegisterServer(void) {
  return AutoIMESelfRegister(g_hInst);
}

STDAPI DllUnregisterServer(void) {
  return AutoIMESelfUnregister();
}
