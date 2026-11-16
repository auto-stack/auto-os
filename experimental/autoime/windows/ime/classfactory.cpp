// AutoIME TSF text service - class factory (PLAN-069 T-02).
#include "ime.h"

#include <new>

STDMETHODIMP CClassFactory::QueryInterface(REFIID riid, void** ppv) {
  *ppv = nullptr;
  if (IsEqualIID(riid, IID_IUnknown) || IsEqualIID(riid, IID_IClassFactory)) {
    *ppv = static_cast<IClassFactory*>(this);
    AddRef();
    return S_OK;
  }
  return E_NOINTERFACE;
}

STDMETHODIMP CClassFactory::CreateInstance(IUnknown* pUnkOuter, REFIID riid,
                                           void** ppvObject) {
  *ppvObject = nullptr;
  if (pUnkOuter) return CLASS_E_NOAGGREGATION;
  auto* service = new (std::nothrow) CTextService();
  if (!service) return E_OUTOFMEMORY;
  const HRESULT hr = service->QueryInterface(riid, ppvObject);
  service->Release();
  return hr;
}

STDMETHODIMP CClassFactory::LockServer(BOOL fLock) {
  AutoIMELockServer(fLock);
  return S_OK;
}
