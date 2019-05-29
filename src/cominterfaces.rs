use crate::Guid;
use crate::ComPtr;
use w::um::unknwnbase::IUnknownVtbl;

/// Marker trait for all COM-compatible interfaces.
pub trait ComInterface {
    /// The type that defines the VTable of this interface.
    type Vtbl: Sized;
    type TAbi: Sized + ComInterfaceAbi;
    unsafe fn wrap_com(ptr: *mut Self::TAbi) -> Self;
    fn get_abi(&self) -> &Self::TAbi;
}

/// Marker trait for all COM-compatible interfaces.
pub trait ComInterfaceAbi {
    /// The type that defines the VTable of this interface.
    type Vtbl: Sized;
}

/// Provides a way to get the IID for a COM/WinRT interface.
/// This should be implemented for all interfaces, except parameterized ones,
/// because IIDs of parameterized interfaces depend on concrete instantiations
/// of the parameter types.
pub trait ComIid {
    // TODO: use associated constant once that is stable
    //const IID: REFIID;
    fn iid() -> &'static Guid;
}

// extend some definitions from winapi (re-export existing types where possible!)
DEFINE_IID!(IID_IUnknown, 0x00000000, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46);

/// Re-export from WinAPI crate
#[allow(non_camel_case_types)]
pub type IUnknown_Abi = w::um::unknwnbase::IUnknown;
#[repr(transparent)]
pub struct IUnknown(ComPtr<IUnknown_Abi>);
impl ComIid for IUnknown { #[inline] fn iid() -> &'static Guid { &IID_IUnknown } }
impl ComInterfaceAbi for IUnknown_Abi { type Vtbl = IUnknownVtbl; }
impl ComInterface for IUnknown {
    type Vtbl = IUnknownVtbl;
    type TAbi = IUnknown_Abi;
    unsafe fn wrap_com(ptr: *mut Self::TAbi) -> Self { IUnknown(ComPtr::wrap_nonnull(ptr)) }
    fn get_abi(&self) -> &Self::TAbi { self.0.as_abi() }
}

impl IUnknown {
    #[inline]
    pub fn query_interface<Target>(&self) -> Option<Target> where Target: ComIid + ComInterface {
        let iid: &'static Guid = Target::iid();
        let mut res = std::ptr::null_mut();
        unsafe {
            match self.get_abi().QueryInterface(iid.as_ref(), &mut res as *mut _ as *mut *mut w::shared::ntdef::VOID) {
                w::shared::winerror::S_OK => Some(Target::wrap_com(res)),
                _ => None
            }
        }
    }
}

DEFINE_IID!(IID_IRestrictedErrorInfo, 0x82BA7092, 0x4C88, 0x427D, 0xA7, 0xBC, 0x16, 0xDD, 0x93, 0xFE, 0xB6, 0x7E);

/// Re-export from WinAPI crate
#[allow(non_camel_case_types)]
pub type IRestrictedErrorInfo_Abi = w::um::restrictederrorinfo::IRestrictedErrorInfo;
#[repr(transparent)]
pub struct IRestrictedErrorInfo(ComPtr<IRestrictedErrorInfo_Abi>);
pub type IRestrictedErrorInfoVtbl = w::um::restrictederrorinfo::IRestrictedErrorInfoVtbl;
impl ComIid for IRestrictedErrorInfo { #[inline] fn iid() -> &'static Guid { &IID_IRestrictedErrorInfo } }
impl ComInterfaceAbi for IRestrictedErrorInfo_Abi { type Vtbl = IRestrictedErrorInfoVtbl; }
impl ComInterface for IRestrictedErrorInfo {
    type Vtbl = IRestrictedErrorInfoVtbl;
    type TAbi = IRestrictedErrorInfo_Abi;
    unsafe fn wrap_com(ptr: *mut Self::TAbi) -> Self { IRestrictedErrorInfo(ComPtr::wrap_nonnull(ptr)) }
    fn get_abi(&self) -> &Self::TAbi { self.0.as_abi() }
}

DEFINE_IID!(IID_IAgileObject, 0x94EA2B94, 0xE9CC, 0x49E0, 0xC0, 0xFF, 0xEE, 0x64, 0xCA, 0x8F, 0x5B, 0x90);

/// Interface that marks an object as agile.
/// It inherits from `IUnknown` and does not have additional members.
#[repr(transparent)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct IAgileObject_Abi {
    lpVtbl: *const IUnknownVtbl // IAgileObject has no methods besides what IUnknown has
}
#[repr(transparent)]
pub struct IAgileObject(ComPtr<IAgileObject_Abi>);
impl std::ops::Deref for IAgileObject {
    type Target = IUnknown;
    #[inline]
    fn deref(&self) -> &IUnknown {
        unsafe { std::mem::transmute(self) }
    }
}
impl std::ops::DerefMut for IAgileObject {
    #[inline]
    fn deref_mut(&mut self) -> &mut IUnknown {
        unsafe { std::mem::transmute(self) }
    }
}
impl ComIid for IAgileObject { #[inline] fn iid() -> &'static Guid { &IID_IAgileObject } }
impl ComInterfaceAbi for IAgileObject_Abi { type Vtbl = IUnknownVtbl; }
impl ComInterface for IAgileObject {
    type Vtbl = IUnknownVtbl;
    type TAbi = IAgileObject_Abi;
    unsafe fn wrap_com(ptr: *mut Self::TAbi) -> Self { IAgileObject(ComPtr::wrap_nonnull(ptr)) }
    fn get_abi(&self) -> &Self::TAbi { self.0.as_abi() }
}
