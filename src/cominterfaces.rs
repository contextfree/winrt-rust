use crate::Guid;
use crate::ComPtr;
use crate::comptr::ComAbi;

/// Trait for all COM-compatible interfaces.
pub trait ComInterface: Sized {
    /// The type that represents the ABI of this interface.
    type TAbi: Sized + ComInterfaceAbi;
    fn get_abi(&self) -> &Self::TAbi;
    #[inline]
    fn get_vtbl(&self) -> &<Self::TAbi as ComInterfaceAbi>::Vtbl {
        unsafe { &*self.get_abi().get_vtbl() }
    }
    unsafe fn wrap_com(ptr: *mut Self::TAbi) -> Self;
    #[inline]
    unsafe fn wrap_com_opt(ptr: *mut Self::TAbi) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self::wrap_com(ptr))
        }
    }
}

/// Trait for all ABI-wrappers of COM-compatible interfaces.
pub trait ComInterfaceAbi {
    /// The type that defines the VTable of this interface.
    type Vtbl: Sized;
    /// Accessor for the VTable pointer
    fn get_vtbl(&self) -> *const Self::Vtbl;
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


#[repr(transparent)]
pub struct IUnknown(ComPtr<w::um::unknwnbase::IUnknown>);
impl ComIid for IUnknown { #[inline] fn iid() -> &'static Guid { &IID_IUnknown } }
impl ComInterfaceAbi for w::um::unknwnbase::IUnknown {
    type Vtbl = w::um::unknwnbase::IUnknownVtbl;
    #[inline]
    fn get_vtbl(&self) -> *const Self::Vtbl {
        self.lpVtbl
    }
}
impl ComInterface for IUnknown {
    type TAbi = w::um::unknwnbase::IUnknown;
    #[inline] unsafe fn wrap_com(ptr: *mut Self::TAbi) -> Self { IUnknown(ComPtr::wrap(ptr)) }
    #[inline] fn get_abi(&self) -> &Self::TAbi { self.0.as_abi() }
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

#[repr(transparent)]
pub struct IRestrictedErrorInfo(ComPtr<ComAbi<w::um::restrictederrorinfo::IRestrictedErrorInfoVtbl>>);
impl ComIid for IRestrictedErrorInfo { #[inline] fn iid() -> &'static Guid { &IID_IRestrictedErrorInfo } }
impl ComInterface for IRestrictedErrorInfo {
    type TAbi = ComAbi<w::um::restrictederrorinfo::IRestrictedErrorInfoVtbl>;
    #[inline] unsafe fn wrap_com(ptr: *mut Self::TAbi) -> Self { IRestrictedErrorInfo(ComPtr::wrap(ptr)) }
    #[inline] fn get_abi(&self) -> &Self::TAbi { self.0.as_abi() }
}

DEFINE_IID!(IID_IAgileObject, 0x94EA2B94, 0xE9CC, 0x49E0, 0xC0, 0xFF, 0xEE, 0x64, 0xCA, 0x8F, 0x5B, 0x90);

#[repr(transparent)]
/// Interface that marks an object as agile.
/// It inherits from `IUnknown` and does not have additional members.
pub struct IAgileObject(ComPtr<ComAbi<w::um::unknwnbase::IUnknownVtbl>>); // IAgileObject has no methods besides what IUnknown has
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
impl ComInterface for IAgileObject {
    type TAbi = ComAbi<w::um::unknwnbase::IUnknownVtbl>;
    #[inline] unsafe fn wrap_com(ptr: *mut Self::TAbi) -> Self { IAgileObject(ComPtr::wrap(ptr)) }
    #[inline] fn get_abi(&self) -> &Self::TAbi { self.0.as_abi() }
}
