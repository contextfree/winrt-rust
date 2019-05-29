use std::ops::{Deref, DerefMut};
use std::fmt;
use std::ptr;
use crate::{ComInterface, ComInterfaceAbi, RtInterface, IInspectable};

use w::shared::minwindef::LPVOID;
use w::um::unknwnbase::IUnknown;
use w::um::combaseapi::CoTaskMemFree;

/// Smart pointer for Windows Runtime objects. This pointer automatically maintains the
/// reference count of the underlying COM object.
#[repr(transparent)]
#[derive(Debug)]
pub(crate) struct ComPtr<T: ComInterfaceAbi>(ptr::NonNull<T>);

impl<T: ComInterfaceAbi> fmt::Pointer for ComPtr<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Pointer::fmt(&self.0, f)
    }
}

pub(crate) trait ComPtrHelpers {
    /// Changes the type of the underlying COM object to a different interface without doing `QueryInterface`.
    /// This is a runtime no-op, but you need to be sure that the interface is compatible.
    unsafe fn into_unchecked<Interface: ComInterface>(self) -> Interface;

    unsafe fn as_unchecked<Interface: ComInterface>(&self) -> &Interface;
}

impl<T> ComPtrHelpers for T where T: ComInterface + Sized {
    #[inline]
    unsafe fn into_unchecked<Interface: ComInterface>(self) -> Interface {
        let ptr = self.get_abi();
        std::mem::forget(self);
        Interface::wrap_com(ptr as *mut _)
    }

    unsafe fn as_unchecked<Interface: ComInterface>(&self) -> &Interface {
        std::mem::transmute(self)
    }
}

pub(crate) fn get_abi<Interface: crate::RtType<In=Interface> + ComInterface>(intf: &Interface) -> <Interface as crate::RtType>::Abi {
    unsafe { Interface::unwrap(intf) }
}

impl<T: ComInterfaceAbi> ComPtr<T> {
    /// Creates a `ComPtr` to wrap a raw pointer.
    /// It takes ownership over the pointer which means it does __not__ call `AddRef`.
    /// The wrapped pointer must not be null.
    #[inline]
    pub unsafe fn wrap_nonnull(ptr: *mut T) -> ComPtr<T> {
        debug_assert!(!ptr.is_null());
        ComPtr(ptr::NonNull::new_unchecked(ptr))
    }

    /// Creates an optional `ComPtr` to wrap a raw pointer that may be null.
    /// It takes ownership over the pointer which means it does __not__ call `AddRef`.
    #[inline]
    pub unsafe fn wrap(ptr: *mut T) -> Option<ComPtr<T>> {
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
    
    /*
    /// Retrieves a `ComPtr` to the specified interface, if it is supported by the underlying object.
    /// If the requested interface is not supported, `None` is returned.
    #[inline]
    pub fn query_interface<Target>(&self) -> Option<ComPtr<Target::TAbi>> where Target: ComIid + ComInterface {
        query_interface::<_, Target>(&*self.as_abi())
    }*/

    // TODO: should be pub(crate)
    #[inline]
    pub fn as_abi(&self) -> &T {
        unsafe { self.0.as_ref() }
    }

    // TODO: should be pub(crate)
    #[inline]
    pub fn as_abi_mut(&mut self) -> &mut T {
        unsafe { self.0.as_mut() }
    }
}

impl<T: ComInterfaceAbi> Clone for ComPtr<T> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe { 
            self.as_unknown().AddRef();
            ComPtr::wrap_nonnull(self.0.as_ptr())
        }
    }
}
impl<T: ComInterfaceAbi> Drop for ComPtr<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { self.as_unknown().Release() };
    }
}
impl<T: ComInterfaceAbi> PartialEq<ComPtr<T>> for ComPtr<T> {
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
    type Target = [T];
    #[inline]
    fn deref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.first.as_ptr() as *mut T, self.size as usize) }
    }
}
impl<T> DerefMut for ComArray<T> where T: crate::RtType {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.first.as_ptr() as *mut T, self.size as usize) }
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
        let p: *mut () = std::ptr::null_mut();
        let _: crate::ComPtr<<crate::IInspectable as crate::ComInterface>::TAbi> = unsafe { std::mem::transmute(p) };
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    #[test]
    fn check_sizes() {
        use std::mem::size_of;

        // make sure that ComPtr is pointer-sized
        assert_eq!(size_of::<crate::ComPtr<<crate::IInspectable as crate::ComInterface>::TAbi>>(), size_of::<<crate::IInspectable as crate::ComInterface>::TAbi>());
        assert_eq!(size_of::<Option<crate::ComPtr<<crate::IInspectable as crate::ComInterface>::TAbi>>>(), size_of::<<crate::IInspectable as crate::ComInterface>::TAbi>());
    }
}
