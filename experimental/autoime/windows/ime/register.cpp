// AutoIME TSF text service - self registration (PLAN-069 T-02).
// Registry shape mirrors Weasel 0.17.4 x64 as observed working on this
// Windows 11 machine (2026-11-16):
//   HKCR\CLSID\{clsid}\InprocServer32 = dll path, ThreadingModel=Apartment
//   HKLM\SOFTWARE\Microsoft\CTF\TIP\{clsid}\Category\Category\{cat}\{clsid}
//   HKLM\SOFTWARE\Microsoft\CTF\TIP\{clsid}\Category\Item\{clsid}\{cat}
//   HKLM\SOFTWARE\Microsoft\CTF\TIP\{clsid}\LanguageProfile\0x0804\{profile}
//       Description/IconFile/IconIndex/Enable=1
// Unregister only deletes AutoIME's own keys. A failed install rolls back
// everything it created in this call.
#include "ime.h"

#include <olectl.h>

#include <string>

namespace {

// Category set of a working keyboard IME (Weasel 0.17.4, 2026-11-16 dump).
const char* const kCategories[] = {
    "{046B8C80-1647-40F7-9B21-B93B81AABC1B}",
    "{13A016DF-560B-46CD-947A-4C3AF1E0E35D}",
    "{24AF3031-852D-40A2-BC09-8992898CE722}",
    "{25504FB4-7BAB-4BC1-9C69-CF81890F0EF5}",
    "{34745C63-B2F0-4784-8B67-5E12C8701A31}",
    "{364215D9-75BC-11D7-A6EF-00065B84435C}",
    "{364215DA-75BC-11D7-A6EF-00065B84435C}",
    "{49D2F9CE-1F5E-11D7-A6D3-00065B84435C}",
    "{49D2F9CF-1F5E-11D7-A6D3-00065B84435C}",
    "{534C48C1-0607-4098-A521-4FC899C73E90}",
    "{565FB8D8-6BD4-4CA1-B223-0F2CCB8F4F96}",
    "{7C6A82AE-B0D7-4F14-A745-14F28B009D61}",
    "{85F9794B-4D19-40D8-8864-4E747371A66D}",
    "{9B7BE3A9-E8AB-4D47-A8FE-254FA423436D}",
    "{B95F181B-EA4C-4AF1-8056-7C321ABBB091}",
    "{CCF05DD7-4A87-11D7-A6E2-00065B84435C}",
};
constexpr int kCategoryCount =
    sizeof(kCategories) / sizeof(kCategories[0]);

constexpr char kClsidKey[] =
    "{02E1E367-4389-429E-B13C-483EB5555D37}";
constexpr char kProfileKey[] =
    "{E8713320-7AE4-40D8-9850-AF2EA17AFD00}";
constexpr char kServiceName[] = "AutoIME";

LONG CreateKey(HKEY root, const char* subkey, HKEY* out) {
  DWORD disposition = 0;
  return RegCreateKeyExA(root, subkey, 0, nullptr, REG_OPTION_NON_VOLATILE,
                         KEY_WRITE, nullptr, out, &disposition);
}

void DeleteKeyTree(HKEY root, const char* subkey) {
  RegDeleteTreeA(root, subkey);
}

std::string GetSelfPath() {
  char path[MAX_PATH]{};
  GetModuleFileNameA(g_hInst, path, MAX_PATH);
  return path;
}

// One registry write step; on failure the caller rolls back everything.
HRESULT WriteClsidRegistration(const std::string& dll_path) {
  const std::string clsid_path =
      std::string("CLSID\\") + kClsidKey;
  HKEY key = nullptr;
  if (CreateKey(HKEY_CLASSES_ROOT, clsid_path.c_str(), &key) !=
      ERROR_SUCCESS)
    return E_FAIL;
  RegSetValueExA(key, nullptr, 0, REG_SZ,
                 reinterpret_cast<const BYTE*>(kServiceName),
                 static_cast<DWORD>(strlen(kServiceName)) + 1);
  RegCloseKey(key);

  const std::string inproc = clsid_path + "\\InprocServer32";
  if (CreateKey(HKEY_CLASSES_ROOT, inproc.c_str(), &key) != ERROR_SUCCESS)
    return E_FAIL;
  RegSetValueExA(key, nullptr, 0, REG_SZ,
                 reinterpret_cast<const BYTE*>(dll_path.c_str()),
                 static_cast<DWORD>(dll_path.size()) + 1);
  const char* threading = "Apartment";
  RegSetValueExA(key, "ThreadingModel", 0, REG_SZ,
                 reinterpret_cast<const BYTE*>(threading),
                 static_cast<DWORD>(strlen(threading)) + 1);
  RegCloseKey(key);
  return S_OK;
}

HRESULT WriteTipRegistration(const std::string& dll_path) {
  const std::string tip_root =
      std::string("SOFTWARE\\Microsoft\\CTF\\TIP\\") + kClsidKey;

  // Language profile zh-CN.
  const std::string profile = tip_root + "\\LanguageProfile\\0x00000804\\" +
                              kProfileKey;
  HKEY key = nullptr;
  if (CreateKey(HKEY_LOCAL_MACHINE, profile.c_str(), &key) != ERROR_SUCCESS)
    return E_FAIL;
  const char* description = "AutoIME";
  RegSetValueExA(key, "Description", 0, REG_SZ,
                 reinterpret_cast<const BYTE*>(description),
                 static_cast<DWORD>(strlen(description)) + 1);
  RegSetValueExA(key, "IconFile", 0, REG_SZ,
                 reinterpret_cast<const BYTE*>(dll_path.c_str()),
                 static_cast<DWORD>(dll_path.size()) + 1);
  const DWORD icon_index = 0;
  RegSetValueExA(key, "IconIndex", 0, REG_DWORD,
                 reinterpret_cast<const BYTE*>(&icon_index),
                 sizeof(icon_index));
  const DWORD enable = 1;
  RegSetValueExA(key, "Enable", 0, REG_DWORD,
                 reinterpret_cast<const BYTE*>(&enable), sizeof(enable));
  RegCloseKey(key);

  // Category <-> item pairs.
  for (int i = 0; i < kCategoryCount; ++i) {
    const std::string cat_key =
        tip_root + "\\Category\\Category\\" + kCategories[i] + "\\" +
        kClsidKey;
    if (CreateKey(HKEY_LOCAL_MACHINE, cat_key.c_str(), &key) != ERROR_SUCCESS)
      return E_FAIL;
    RegCloseKey(key);
    const std::string item_key =
        tip_root + "\\Category\\Item\\" + kClsidKey + "\\" + kCategories[i];
    if (CreateKey(HKEY_LOCAL_MACHINE, item_key.c_str(), &key) !=
        ERROR_SUCCESS)
      return E_FAIL;
    RegCloseKey(key);
  }
  return S_OK;
}

void RollbackAutoIMEKeys() {
  const std::string tip =
      std::string("SOFTWARE\\Microsoft\\CTF\\TIP\\") + kClsidKey;
  DeleteKeyTree(HKEY_LOCAL_MACHINE, tip.c_str());
  DeleteKeyTree(HKEY_CLASSES_ROOT,
                (std::string("CLSID\\") + kClsidKey).c_str());
}

}  // namespace

HRESULT AutoIMESelfRegister(HINSTANCE) {
  ImeLog("DllRegisterServer begin");
  const std::string dll_path = GetSelfPath();

  HRESULT hr = WriteClsidRegistration(dll_path);
  if (SUCCEEDED(hr)) hr = WriteTipRegistration(dll_path);
  if (FAILED(hr)) {
    // Install failure: undo everything this call added.
    ImeLog("DllRegisterServer FAILED hr=0x%08lx, rolling back",
           static_cast<unsigned long>(hr));
    RollbackAutoIMEKeys();
    return SELFREG_E_CLASS;  // surface failure to regsvr32
  }
  ImeLog("DllRegisterServer ok dll=%s", dll_path.c_str());
  return S_OK;
}

HRESULT AutoIMESelfUnregister() {
  ImeLog("DllUnregisterServer begin");
  RollbackAutoIMEKeys();
  ImeLog("DllUnregisterServer ok");
  return S_OK;
}
