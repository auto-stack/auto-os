// AutoIME - TSF registration verifier (PLAN-069 T-02).
// Programmatic oracle for "the input switcher can list AutoIME": enumerates
// the enabled zh-CN language profiles via ITfInputProcessorProfiles (the same
// store the switcher reads) and reports whether AutoIME's CLSID + profile are
// present and enabled.
//   verify_registered.exe        exit 0 = registered+enabled, 1 = absent,
//                                2 = COM/TSF failure
#include <initguid.h>
#include "ime.h"

#include <objbase.h>

#include <cstdio>

int main() {
  setvbuf(stdout, nullptr, _IONBF, 0);
  if (FAILED(CoInitialize(nullptr))) {
    printf("ERROR CoInitialize failed\n");
    return 2;
  }
  ITfInputProcessorProfiles* profiles = nullptr;
  HRESULT hr = CoCreateInstance(CLSID_TF_InputProcessorProfiles, nullptr,
                                CLSCTX_INPROC_SERVER, IID_ITfInputProcessorProfiles,
                                reinterpret_cast<void**>(&profiles));
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
  printf("RESULT AutoIME registered=%d enabled=%d (Enable reg=%ld)\n", 1,
         enabled ? 1 : 0, st == ERROR_SUCCESS ? static_cast<long>(enable) : -1L);
  return enabled ? 0 : 1;
}
