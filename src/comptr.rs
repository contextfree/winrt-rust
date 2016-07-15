use std::ops::{Deref, DerefMut};
use std::fmt;
use std::ptr;
use ::{ComIid, ComInterface, RtInterface, IInspectable, Guid};

#[derive(Debug)]
pub struct ComPtr<T>(*mut T); // TODO: use NonZero?

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
        match as_unknown.QueryInterface(&iid.as_iid(), &mut res as *mut _ as *mut *mut ::w::VOID) {
            ::w::S_OK => Some(ComPtr::wrap(res)),
            _ => None
        }
    }
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
        unsafe { self.as_unknown().Release(); }
    }
}
impl<T> PartialEq<ComPtr<T>> for ComPtr<T> {
    fn eq(&self, other: &ComPtr<T>) -> bool {
        self.0 == other.0
    }
}

/// Owned array type that will be deallocated using `CoTaskMemFree` on drop.
pub struct ComArray<T> {
    size: u32,
    first: *mut T
}

impl<T> ComArray<T> {
    #[inline]
    pub unsafe fn from_raw(size: u32, first: *mut T) -> ComArray<T> {
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

impl<T> Deref for ComArray<T> {
    type Target = [T];
    #[inline]
    fn deref(&self) -> &[T] {
        unsafe { ::std::slice::from_raw_parts(self.first, self.size as usize) }
    }
}
impl<T> DerefMut for ComArray<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { ::std::slice::from_raw_parts_mut(self.first, self.size as usize) }
    }
}

impl<T> Drop for ComArray<T> {
    #[inline]
    fn drop(&mut self) {
        // TODO: call `Release` on elements if T is an interface reference
        unsafe {
            //println!("Dropping ComArray");
            ::std::ptr::drop_in_place(&mut self[..]);
            ::ole32::CoTaskMemFree(self.first as ::w::LPVOID)
        };
    }
}