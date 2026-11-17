// AutoIME - TSF registration verifier (PLAN-069 T-02, PLAN-070 T-04/F5).
// Programmatic oracle for "the input switcher can list AutoIME and points at
// the installed binary": enumerates the enabled zh-CN language profiles via
// ITfInputProcessorProfiles (the same store the switcher reads), then checks
// the CLSID InprocServer32 path, that the file exists, and its file version.
//
// Usage: verify_registered.exe [expected\autoime_ime.dll]
// Exit codes: 0 registered+enabled+path/file/version ok, 1 not registered or
// disabled, 4 registered path/file mismatch, 2 COM/TSF failure.
#include <initguid.h>
#include "ime.h"

#include <objbase.h>

#include <cstdio>
#include <cstring>
#include <string>
#include <vector>

namespace {

constexpr char kClsidKey[] = "{02E1E367-4389-429E-B13C-483EB5555D37}";

bool FileExists(const std::string& path) {
  const DWORD attrs = GetFileAttributesA(path.c_str());
  return attrs != INVALID_FILE_ATTRIBUTES &&
         !(attrs & FILE_ATTRIBUTE_DIRECTORY);
}

std::string ReadInprocServer32() {
  char value[MAX_PATH * 4]{};
  DWORD size = sizeof(value);
  const std::string key =
      std::string("CLSID\\") + kClsidKey + "\\InprocServer32";
  const LSTATUS st = RegGetValueA(HKEY_CLASSES_ROOT, key.c_str(), nullptr,
                                  RRF_RT_REG_SZ, nullptr, value, &size);
  return st == ERROR_SUCCESS ? std::string(value) : std::string();
}

std::string FileVersion(const std::string& path) {
  DWORD handle = 0;
  const DWORD size = GetFileVersionInfoSizeA(path.c_str(), &handle);
  if (size == 0) return "n/a";
  std::vector<char> data(size);
  if (!GetFileVersionInfoA(path.c_str(), handle, size, data.data()))
    return "n/a";
  VS_FIXEDFILEINFO* info = nullptr;
  UINT len = 0;
  if (!VerQueryValueA(data.data(), "\\", reinterpret_cast<LPVOID*>(&info),
                      &len) ||
      info == nullptr)
    return "n/a";
  char out[64];
  std::snprintf(out, sizeof(out), "%u.%u.%u.%u",
                HIWORD(info->dwFileVersionMS), LOWORD(info->dwFileVersionMS),
                HIWORD(info->dwFileVersionLS), LOWORD(info->dwFileVersionLS));
  return out;
}

bool SamePath(const std::string& a, const std::string& b) {
  return _stricmp(a.c_str(), b.c_str()) == 0;
}

}  // namespace

int main(int argc, char** argv) {
  setvbuf(stdout, nullptr, _IONBF, 0);
  if (FAILED(CoInitialize(nullptr))) {
    printf("ERROR CoInitialize failed\n");
    return 2;
  }
  ITfInputProcessorProfiles* profiles = nullptr;
  HRESULT hr = CoCreateInstance(
      CLSID_TF_InputProcessorProfiles, nullptr, CLSCTX_INPROC_SERVER,
      IID_ITfInputProcessorProfiles, reinterpret_cast<void**>(&profiles));
  if (FAILED(hr)) {
    printf("ERROR CoCreateInstance(TF_InputProcessorProfiles) hr=0x%08lx\n",
           static_cast<unsigned long>(hr));
    CoUninitialize();
    return 2;
  }

  IEnumTfLanguageProfiles* en = nullptr;
  hr = profiles->EnumLanguageProfiles(kAutoIMElangid, &en);
  if (FAILED(hr)) {
    printf("ERROR EnumLanguageProfiles(0x0804) hr=0x%08lx\n",
           static_cast<unsigned long>(hr));
    profiles->Release();
    CoUninitialize();
    return 2;
  }

  TF_LANGUAGEPROFILE entry{};
  bool found = false;
  while (en->Next(1, &entry, nullptr) == S_OK) {
    if (IsEqualCLSID(entry.clsid, CLSID_AutoIMETextService)) {
      found = true;
      wchar_t clsid_str[64] = L"";
      StringFromGUID2(entry.clsid, clsid_str, 64);
      wchar_t profile_str[64] = L"";
      StringFromGUID2(entry.guidProfile, profile_str, 64);
      printf("profile clsid=%S guid=%S langid=0x%04x\n", clsid_str,
             profile_str, entry.langid);
    }
  }
  en->Release();
  profiles->Release();
  CoUninitialize();

  if (!found) {
    printf("RESULT AutoIME not registered for 0x0804\n");
    return 1;
  }
  // The switcher's enable state lives in the profile's Enable value.
  char key_path[256];
  snprintf(key_path, sizeof(key_path),
           "SOFTWARE\\Microsoft\\CTF\\TIP\\"
           "{02E1E367-4389-429E-B13C-483EB5555D37}\\LanguageProfile\\0x"
           "00000804\\{E8713320-7AE4-40D8-9850-AF2EA17AFD00}");
  DWORD enable = 0, size = sizeof(enable);
  const LSTATUS st = RegGetValueA(HKEY_LOCAL_MACHINE, key_path, "Enable",
                                  RRF_RT_REG_DWORD, nullptr, &enable, &size);
  const bool enabled = st == ERROR_SUCCESS && enable != 0;

  // PLAN-070 T-04/F5: verify the registered path, the file and its version.
  const std::string registered = ReadInprocServer32();
  const bool have_path = !registered.empty();
  const bool file_ok = have_path && FileExists(registered);
  const std::string version = file_ok ? FileVersion(registered) : "n/a";
  const bool version_ok = version != "n/a";
  bool expected_ok = true;
  if (argc > 1 && argv[1] && argv[1][0])
    expected_ok = SamePath(registered, argv[1]);
  printf("RESULT AutoIME registered=%d enabled=%d path=\"%s\" file=%d "
         "version=%s path_matches_expected=%d\n",
         1, enabled ? 1 : 0, registered.c_str(), file_ok ? 1 : 0,
         version.c_str(), expected_ok ? 1 : 0);
  const bool ok = enabled && have_path && file_ok && version_ok && expected_ok;
  return ok ? 0 : 4;
}
