use std::ops::{Deref, DerefMut};
use std::fmt;
use std::mem;
use super::{ComIid, ComGetPtr, RtInterface, IInspectable, HString, out};

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
    pub unsafe fn new(ptr: *mut T) -> ComPtr<T> { ComPtr(ptr) }
    
    /// Creates an uninitialized `ComPtr`. Unsafe because it needs to be initialized
    /// or else `Drop` will try to call `Release` on a null pointer.
    pub unsafe fn uninitialized() -> ComPtr<T> { ComPtr(::std::ptr::null_mut()) }

    fn as_inspectable(&self) -> &mut IInspectable where T: RtInterface {
        unsafe { &mut *(self.0 as *mut IInspectable) }
    }

    pub fn get_runtime_class_name(&self) -> HString where T: RtInterface {
        let mut result = HString::empty();
        let hres = unsafe { self.as_inspectable().GetRuntimeClassName(out(&mut result)) };
        assert_eq!(hres, ::w::S_OK);
        result
    }
    
    fn as_unknown(&self) -> &mut ::w::IUnknown {
        unsafe { &mut *(self.0 as *mut ::w::IUnknown) }
    }
    
    pub fn query_interface<Target>(&self) -> Option<ComPtr<Target>> where Target: ComIid {
        //let iid: ::w::REFIID = Target::IID;
        let iid: ::w::REFIID = Target::get_iid();
        let mut result: ComPtr<Target> = unsafe { ComPtr::uninitialized() };
        match unsafe { self.as_unknown().QueryInterface(iid, result.get_address() as *mut _ as *mut *mut ::w::VOID) } {
            ::w::S_OK => Some(result),
            _ => {
                mem::forget(result); // this pointer is still null, so we have to prevent drop
                None
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
            ComPtr::new(self.0)
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

// TODO: remove the get_address methods from the inherent impls and only use the trait?
impl<T> ComGetPtr for ComPtr<T> {
    type Abi = *mut T;
    fn get_address(&mut self) -> &mut Self::Abi {
        &mut self.0
    }
}