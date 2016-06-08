use std::ops::{Deref, DerefMut};
use std::fmt;
use std::ptr;
use super::{ComIid, RtInterface, IInspectable, HString};

#[derive(Debug)]
pub struct ComPtr<T>(*mut T); // TODO: use NonZero?

impl<T> fmt::Pointer for ComPtr<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Pointer::fmt(&self.0, f)
    }
}

impl<T> ComPtr<T> {
    /// Creates a `ComPtr` to wrap a raw pointer.
    /// It takes ownership over the pointer which means it does __not__ call `AddRef`.
    /// `T` __must__ be a COM interface that inherits from `IUnknown`.
    pub unsafe fn wrap(ptr: *mut T) -> ComPtr<T> {
        debug_assert!(!ptr.is_null());
        ComPtr(ptr)
    }

    fn as_inspectable(&self) -> &mut IInspectable where T: RtInterface {
        unsafe { &mut *(self.0 as *mut IInspectable) }
    }

    // TODO: it seems to be disallowed to call this on "...Statics" objects (E_ILLEGAL_METHOD_CALL)
    //       -> can we prevent that at compile time?
    pub fn get_runtime_class_name(&self) -> HString where T: RtInterface {
        let mut result = ptr::null_mut();
        let hres = unsafe { self.as_inspectable().GetRuntimeClassName(&mut result) };
        assert_eq!(hres, ::w::S_OK);
        unsafe { HString::wrap(result) }
    }
    
    fn as_unknown(&self) -> &mut ::w::IUnknown {
        unsafe { &mut *(self.0 as *mut ::w::IUnknown) }
    }
    
    pub fn query_interface<Target>(&self) -> Option<ComPtr<Target>> where Target: ComIid {
        //let iid: ::w::REFIID = Target::IID;
        let iid: ::w::REFIID = Target::get_iid();
        let mut res = ptr::null_mut();
        unsafe {
            match self.as_unknown().QueryInterface(iid, &mut res as *mut _ as *mut *mut ::w::VOID) {
                ::w::S_OK => Some(ComPtr::wrap(res)),
                _ => None
            }
        }
    }
}
impl<T> Deref for ComPtr<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.0 }
    }
}
impl<T> DerefMut for ComPtr<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut*self.0 }
    }
}
impl<T> Clone for ComPtr<T> {
    fn clone(&self) -> Self {
        unsafe { 
            self.as_unknown().AddRef();
            ComPtr::wrap(self.0)
        }
    }
}
impl<T> Drop for ComPtr<T> {
    fn drop(&mut self) {
        unsafe { self.as_unknown().Release(); }
    }
}
impl<T> PartialEq<ComPtr<T>> for ComPtr<T> {
    fn eq(&self, other: &ComPtr<T>) -> bool {
        self.0 == other.0
    }
}