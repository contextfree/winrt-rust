use winapi as w;
use std::ops::{Deref, DerefMut};
use std::fmt;
use std::{ptr, mem};
use super::ComInterfaceIid;

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
    
    /// Casts up the inheritance chain
    pub fn up<U>(&self) -> ComPtr<U> where T: Deref<Target=U> {
        unimplemented!() // TODO
    }
    /// Make sure you know what you're doing with this function
    pub unsafe fn as_mut(&self) -> &mut T {
        &mut*self.0
    }
    fn as_unknown(&self) -> &mut w::IUnknown {
        unsafe { &mut *(self.0 as *mut w::IUnknown) }
    }
    
    pub unsafe fn get_address(&mut self) -> &mut *mut T {
        &mut self.0
    }
    
    pub fn query_interface<Target>(&self) -> Option<ComPtr<Target>> where Target: ComInterfaceIid {
        //let iid: ::winapi::REFIID = Target::IID;
        let iid: ::winapi::REFIID = Target::get_iid();
        let mut result: ComPtr<Target> = unsafe { ComPtr::new(ptr::null_mut()) };
        match unsafe { self.as_unknown().QueryInterface(iid, result.get_address() as *mut _ as *mut *mut ::winapi::VOID) } {
            ::winapi::S_OK => Some(result),
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
