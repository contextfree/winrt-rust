use std::ops::{Deref, DerefMut};
use std::fmt;
use std::ptr;
use ::{ComIid, ComInterface, RtInterface, RtClassInterface, IInspectable, Guid};

#[repr(C)] #[derive(Debug)]
pub struct ComPtr<T>(*mut T); // TODO: use NonZero or Shared (see https://github.com/rust-lang/rust/issues/27730)

impl<T> fmt::Pointer for ComPtr<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Pointer::fmt(&self.0, f)
    }
}

pub fn query_interface<T, Target>(interface: &T) -> Option<ComPtr<Target>> where Target: ComIid, T: ComInterface {
    let iid: &'static Guid = Target::iid();
    let as_unknown = unsafe { &mut *(interface  as *const T as *mut T as *mut ::w::IUnknown) };
    let mut res = ptr::null_mut();
    unsafe {
        match as_unknown.QueryInterface(iid.as_ref(), &mut res as *mut _ as *mut *mut ::w::VOID) {
            ::w::S_OK => Some(ComPtr::wrap(res)),
            _ => None
        }
    }
}

// This trait is not exported in the library interface
pub trait HiddenGetRuntimeClassName {
    fn get_runtime_class_name(&self) -> ::HString;
}

impl<T> ComPtr<T> {
    /// Creates a `ComPtr` to wrap a raw pointer.
    /// It takes ownership over the pointer which means it does __not__ call `AddRef`.
    /// `T` __must__ be a COM interface that inherits from `IUnknown`.
    pub unsafe fn wrap(ptr: *mut T) -> ComPtr<T> { // TODO: Add T: ComInterface bound
        debug_assert!(!ptr.is_null());
        ComPtr(ptr)
    }

    fn as_inspectable(&self) -> &mut IInspectable where T: RtInterface {
        unsafe { &mut *(self.0 as *mut IInspectable) }
    }
    
    fn as_unknown(&self) -> &mut ::w::IUnknown {
        unsafe { &mut *(self.0 as *mut ::w::IUnknown) }
    }
    
    pub fn get_runtime_class_name(&self) -> ::HString where T: RtClassInterface {
        HiddenGetRuntimeClassName::get_runtime_class_name(self.as_inspectable())
    }
    
    pub fn query_interface<Target>(&self) -> Option<ComPtr<Target>> where Target: ComIid, T: ComInterface {
        query_interface::<_, Target>(&**self)
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
        unsafe { self.as_unknown().Release() };
    }
}
impl<T> PartialEq<ComPtr<T>> for ComPtr<T> {
    fn eq(&self, other: &ComPtr<T>) -> bool {
        self.0 == other.0
    }
}

/// Owned array type that is returned from WinRT calls.
/// Has been allocated by WinRT and will be deallocated using `CoTaskMemFree` on drop.
pub struct ComArray<T> where T: ::RtType {
    size: u32,
    first: *mut T::Abi
}

impl<T> ComArray<T> where T: ::RtType {
    #[inline]
    pub unsafe fn from_raw(size: u32, first: *mut T::Abi) -> ComArray<T> {
        assert!(!first.is_null());
        ComArray {
            size: size,
            first: first
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.size as usize
    }
}

impl<T> Deref for ComArray<T> where T: ::RtType {
    type Target = [T::Out];
    #[inline]
    fn deref(&self) -> &[T::Out] {
        unsafe { ::std::slice::from_raw_parts(self.first as *mut T::Out, self.size as usize) }
    }
}
impl<T> DerefMut for ComArray<T> where T: ::RtType {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T::Out] {
        unsafe { ::std::slice::from_raw_parts_mut(self.first as *mut T::Out, self.size as usize) }
    }
}

impl<T> Drop for ComArray<T> where T: ::RtType {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ::std::ptr::drop_in_place(&mut self[..]);
            ::ole32::CoTaskMemFree(self.first as ::w::LPVOID)
        };
    }
}

mod extra {
    // makes sure that compile fails when ComPtr is not pointer-sized
    // i.e. when a compiler version is used that still has dropflags
    fn assert_no_dropflags() {
        let p: *mut ::IInspectable = ::std::ptr::null_mut();
        let _: ::ComPtr<::IInspectable> = unsafe { ::std::mem::transmute(p) };
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    #[test]
    fn check_sizes() {
        use ::std::mem::size_of;

        // make sure that ComPtr is pointer-sized
        assert_eq!(size_of::<::ComPtr<::IInspectable>>(), size_of::<*mut ::IInspectable>());
        
        // TODO: enable this once the null-pointer optimization can be used for Option<ComPtr>
        //assert_eq!(size_of::<Option<::ComPtr<::IInspectable>>>(), size_of::<*mut ::IInspectable>());
    }
}