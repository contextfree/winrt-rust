use std::ops::{Deref, DerefMut};
use std::fmt;
use std::ptr;
use crate::{ComIid, ComInterface, RtInterface, RtClassInterface, IInspectable, Guid};

use w::shared::ntdef::VOID;
use w::shared::minwindef::LPVOID;
use w::shared::winerror::S_OK;
use w::um::unknwnbase::IUnknown;
use w::um::combaseapi::CoTaskMemFree;

/// Smart pointer for Windows Runtime objects. This pointer automatically maintains the
/// reference count of the underlying COM object.
#[repr(transparent)]
#[derive(Debug)]
pub struct ComPtr<T>(ptr::NonNull<T>);

impl<T> fmt::Pointer for ComPtr<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Pointer::fmt(&self.0, f)
    }
}

// This is a helper method that is not exposed publically by the library
#[inline]
pub fn query_interface<T, Target>(interface: &T) -> Option<ComPtr<Target>> where Target: ComIid, T: ComInterface {
    let iid: &'static Guid = Target::iid();
    let as_unknown = unsafe { &mut *(interface  as *const T as *mut T as *mut IUnknown) };
    let mut res = ptr::null_mut();
    unsafe {
        match as_unknown.QueryInterface(iid.as_ref(), &mut res as *mut _ as *mut *mut VOID) {
            S_OK => Some(ComPtr::wrap(res)),
            _ => None
        }
    }
}

// This trait is not exported in the library interface
pub trait HiddenGetRuntimeClassName {
    fn get_runtime_class_name(&self) -> crate::HString;
}

impl<T> ComPtr<T> {
    /// Creates a `ComPtr` to wrap a raw pointer.
    /// It takes ownership over the pointer which means it does __not__ call `AddRef`.
    /// `T` __must__ be a COM interface that inherits from `IUnknown`.
    /// The wrapped pointer must not be null.
    #[inline]
    pub unsafe fn wrap(ptr: *mut T) -> ComPtr<T> { // TODO: Add T: ComInterface bound
        debug_assert!(!ptr.is_null());
        ComPtr(ptr::NonNull::new_unchecked(ptr))
    }

    /// Creates an optional `ComPtr` to wrap a raw pointer that may be null.
    /// It takes ownership over the pointer which means it does __not__ call `AddRef`.
    /// `T` __must__ be a COM interface that inherits from `IUnknown`.
    #[inline]
    pub unsafe fn wrap_optional(ptr: *mut T) -> Option<ComPtr<T>> { // TODO: Add T: ComInterface bound
        if ptr.is_null() {
            None
        } else {
            Some(ComPtr(ptr::NonNull::new_unchecked(ptr)))
        }
    }

    /// Returns the underlying WinRT object as a reference to an `IInspectable` object.
    #[inline]
    fn as_inspectable(&self) -> &mut IInspectable where T: RtInterface {
        unsafe { &mut *(self.0.as_ptr() as *mut IInspectable) }
    }
    
    /// Returns the underlying WinRT or COM object as a reference to an `IUnknown` object.
    #[inline]
    fn as_unknown(&self) -> &mut IUnknown {
        unsafe { &mut *(self.0.as_ptr() as *mut IUnknown) }
    }

    /// Changes the type of the underlying COM object to a different interface without doing `QueryInterface`.
    /// This is a runtime no-op, but you need to be sure that the interface is compatible.
    #[inline]
    pub unsafe fn into_unchecked<Interface>(self) -> ComPtr<Interface> where Interface: ComInterface {
        std::mem::transmute(self)
    }
    
    /// Gets the fully qualified name of the current Windows Runtime object.
    /// This is only available for interfaces that inherit from `IInspectable` and
    /// are not factory or statics interfaces.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use winrt::*;
    /// use winrt::windows::foundation::Uri;
    ///
    /// let uri = FastHString::new("https://www.rust-lang.org");
    /// let uri = Uri::create_uri(&uri).unwrap();
    /// assert_eq!("Windows.Foundation.Uri", uri.get_runtime_class_name().to_string());
    /// ```
    #[inline]
    pub fn get_runtime_class_name(&self) -> crate::HString where T: RtClassInterface {
        HiddenGetRuntimeClassName::get_runtime_class_name(self.as_inspectable())
    }
    
    /// Retrieves a `ComPtr` to the specified interface, if it is supported by the underlying object.
    /// If the requested interface is not supported, `None` is returned.
    #[inline]
    pub fn query_interface<Target>(&self) -> Option<ComPtr<Target>> where Target: ComIid, T: ComInterface {
        query_interface::<_, Target>(&**self)
    }
}
impl<T> Deref for ComPtr<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}
impl<T> DerefMut for ComPtr<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.0.as_mut() }
    }
}
impl<T> Clone for ComPtr<T> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe { 
            self.as_unknown().AddRef();
            ComPtr::wrap(self.0.as_ptr())
        }
    }
}
impl<T> Drop for ComPtr<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { self.as_unknown().Release() };
    }
}
impl<T> PartialEq<ComPtr<T>> for ComPtr<T> {
    #[inline]
    fn eq(&self, other: &ComPtr<T>) -> bool {
        self.0 == other.0
    }
}

/// Owned array type that is used as return type when WinRT methods return arrays.
/// It wraps a block of memory that has been allocated by WinRT and will be deallocated
/// using `CoTaskMemFree` on drop.
pub struct ComArray<T> where T: crate::RtType {
    size: u32,
    first: ptr::NonNull<T::Abi>
}

impl<T> ComArray<T> where T: crate::RtType {
    #[inline]
    pub unsafe fn from_raw(size: u32, first: *mut T::Abi) -> ComArray<T> {
        assert!(!first.is_null());
        ComArray {
            size: size,
            first: ptr::NonNull::new_unchecked(first)
        }
    }

    /// Returns the length of the array.
    #[inline]
    pub fn len(&self) -> usize {
        self.size as usize
    }
}

impl<T> Deref for ComArray<T> where T: crate::RtType {
    type Target = [T::OutNonNull];
    #[inline]
    fn deref(&self) -> &[T::OutNonNull] {
        unsafe { std::slice::from_raw_parts(self.first.as_ptr() as *mut T::OutNonNull, self.size as usize) }
    }
}
impl<T> DerefMut for ComArray<T> where T: crate::RtType {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T::OutNonNull] {
        unsafe { std::slice::from_raw_parts_mut(self.first.as_ptr() as *mut T::OutNonNull, self.size as usize) }
    }
}

impl<T> Drop for ComArray<T> where T: crate::RtType {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(&mut self[..]);
            CoTaskMemFree(self.first.as_ptr() as LPVOID)
        };
    }
}

mod extra {
    // makes sure that compile fails when ComPtr is not pointer-sized
    // i.e. when a compiler version is used that still has dropflags
    #[inline]
    fn assert_no_dropflags() {
        let p: *mut crate::IInspectable = std::ptr::null_mut();
        let _: crate::ComPtr<crate::IInspectable> = unsafe { std::mem::transmute(p) };
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    #[test]
    fn check_sizes() {
        use std::mem::size_of;

        // make sure that ComPtr is pointer-sized
        assert_eq!(size_of::<crate::ComPtr<crate::IInspectable>>(), size_of::<*mut crate::IInspectable>());
        assert_eq!(size_of::<Option<crate::ComPtr<crate::IInspectable>>>(), size_of::<*mut crate::IInspectable>());
    }
}
