use std::ptr;

use super::{ComInterface, HString, HStringReference, HStringArg, ComPtr, ComArray, ComIid, Guid};
use crate::HRESULT;

use w::shared::ntdef::{VOID, ULONG};
use w::shared::winerror::{S_OK, S_FALSE, CO_E_NOTINITIALIZED, REGDB_E_CLASSNOTREG};
use w::shared::guiddef::IID;
use w::um::unknwnbase::IUnknownVtbl;
use w::winrt::hstring::HSTRING;
use w::winrt::roapi::{RO_INIT_MULTITHREADED, RO_INIT_SINGLETHREADED, RoInitialize, RoUninitialize, RoGetActivationFactory};
use w::um::combaseapi::CoIncrementMTAUsage;

use self::gen::windows::foundation::collections::{
    IIterable,
    IIterator,
    IVector,
    IVectorView,
    IObservableVector
};

/// Represents a single UTF-16 character. This is the standard character type in WinRT. 
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Char(pub w::ctypes::wchar_t); // TODO: deref to u16

/// Marker trait for all Windows Runtime interfaces. They must inherit from `IInspectable`.
pub unsafe trait RtInterface: ComInterface {}

/// Marker trait for all interfaces that are not factories or statics.
pub unsafe trait RtClassInterface: RtInterface + Sized {
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
    fn get_runtime_class_name(&self) -> crate::HString {
        use crate::comptr::ComPtrHelpers;
        IInspectable::get_runtime_class_name(unsafe { self.as_unchecked::<IInspectable>() })
    }
}

/// Marker trait for all value types (primitive types, structs, enums) that can be used as
/// generic parameters in Windows Runtime.
pub unsafe trait RtValueType: Copy {}

unsafe impl RtValueType for bool {}
unsafe impl RtValueType for f32 {}
unsafe impl RtValueType for f64 {}
unsafe impl RtValueType for i8 {}
unsafe impl RtValueType for i16 {}
unsafe impl RtValueType for i32 {}
unsafe impl RtValueType for i64 {}
unsafe impl RtValueType for u8 {}
unsafe impl RtValueType for u16 {}
unsafe impl RtValueType for u32 {}
unsafe impl RtValueType for u64 {}
unsafe impl RtValueType for Char {}
unsafe impl RtValueType for Guid {}

/// This is a trait implemented by all types that can be used as generic parameters of parameterized interfaces.
/// `Abi`, `Self` and `Out` must be binary compatible (i.e. `wrap` must basically be the same as `transmute`)
/// in order to work in `ComArray`.
pub trait RtType {
    type In;
    type Abi;
    type Out;

    unsafe fn unwrap(input: &Self::In) -> Self::Abi;
    unsafe fn uninitialized() -> Self::Abi;
    unsafe fn wrap(abi: Self::Abi) -> Self::Out;
    unsafe fn wrap_nonnull(abi: Self::Abi) -> Self;
}

impl<'a> RtType for HString {
    type In = HStringArg;
    type Abi = HSTRING;
    type Out = Self;

    #[doc(hidden)] #[inline]
    unsafe fn unwrap(v: &HStringArg) -> Self::Abi {
        v.get()
    }
    #[doc(hidden)] #[inline]
    unsafe fn uninitialized() -> Self::Abi {
        std::ptr::null_mut()
    }
    #[doc(hidden)] #[inline]
    unsafe fn wrap(v: Self::Abi) -> Self::Out {
        HString::wrap(v)
    }
    #[doc(hidden)] #[inline]
    unsafe fn wrap_nonnull(v: Self::Abi) -> Self {
        HString::wrap(v)
    }
}

impl<T> RtType for T where T: RtValueType
{
    type In = Self;
    type Abi = Self;
    type Out = Self;

    #[doc(hidden)] #[inline]
    unsafe fn unwrap(v: &Self::In) -> Self::Abi {
        *v
    }
    #[doc(hidden)] #[inline]
    unsafe fn uninitialized() -> Self::Abi {
        std::mem::zeroed()
    }
    #[doc(hidden)] #[inline]
    unsafe fn wrap(v: Self::Abi) -> Self::Out {
        v
    }
    #[doc(hidden)] #[inline]
    unsafe fn wrap_nonnull(v: Self::Abi) -> Self {
        v
    }
}

/// This trait is implemented by all classes that have a name
/// associated with them which can be used at runtime.
pub trait RtNamedClass {
    /// Returns the name of the class encoded as a 16-bit wide string.
    fn name() -> &'static [u16];
}

pub trait RtActivatable<Interface> : RtNamedClass {
    /// Returns a factory object to create instances of this class or to call static methods.
    #[inline]
    fn get_activation_factory() -> Interface where Interface: RtInterface + ComIid + RtType, Interface: RtType<Abi=*mut <Interface as ComInterface>::TAbi> {
        let mut res: <Interface as RtType>::Abi = unsafe { <Interface as RtType>::uninitialized() };
        let class_id = unsafe { HStringReference::from_utf16_unchecked(Self::name()) };
        let mut hr = unsafe { RoGetActivationFactory(class_id.get(), <Interface as ComIid>::iid().as_ref(), &mut res as *mut *mut _ as *mut *mut VOID) };
        if hr == CO_E_NOTINITIALIZED {
            let mut cookie = ptr::null_mut();
            unsafe { CoIncrementMTAUsage(&mut cookie); }
            hr = unsafe { RoGetActivationFactory(class_id.get(), <Interface as ComIid>::iid().as_ref(), &mut res as *mut *mut _ as *mut *mut VOID) };
        }
        if hr == S_OK {
            unsafe { <Interface as RtType>::wrap_nonnull(res) }
        } else if hr == REGDB_E_CLASSNOTREG {
            let name = Self::name();
            panic!("WinRT class \"{}\" not registered", String::from_utf16_lossy(&name[0..name.len()-1]))
        } else {
            panic!("RoGetActivationFactory failed with error code 0x{:X}", hr as u32)
        }     
    }
}

/// Enables easy access to a default constructor, using `IActivationFactory` under the hood.
pub trait RtDefaultConstructible {
    /// Uses the default constructor to create an instance of this class.
    fn new() -> Self where Self: Sized + RtActivatable<IActivationFactory> + ComInterface;
}

impl<T> RtDefaultConstructible for T where T: RtActivatable<IActivationFactory> {
    #[inline]
    fn new() -> Self where Self: Sized + RtActivatable<IActivationFactory> + ComInterface {
        use crate::comptr::ComPtrHelpers;
        let factory: IActivationFactory = Self::get_activation_factory();
        unsafe { factory.activate_instance().into_unchecked() }
    }
}

pub struct IteratorAdaptor<'a, T: RtType + 'a> {
    iter: IIterator<T>,
    called_next: bool,
    phantom: std::marker::PhantomData<&'a IIterable<T>>
}

impl<'a, T: RtType + 'a> IteratorAdaptor<'a, T> {
    // careful: this creates an unbounded lifetime
    fn new(iter: IIterator<T>) -> Self {
        IteratorAdaptor {
            iter: iter,
            called_next: false,
            phantom: std::marker::PhantomData
        }
    }
}

impl<'a, T> Iterator for IteratorAdaptor<'a, T> where T: RtType
{
    type Item = <T as RtType>::Out;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if !self.called_next {
            self.called_next = true;
            match self.iter.get_has_current().expect("IIterator::get_has_current failed") {
                true => Some(self.iter.get_current().expect("IIterator::get_current failed")),
                false => None
            }
        } else {
            match self.iter.move_next() {
                Ok(true) => Some(self.iter.get_current().expect("IIterator::get_current failed")),
                Ok(false) => None,
                Err(crate::Error::ChangedState) => panic!("the iterator was invalidated by an operation that changed the state of the container"),
                Err(e) => panic!("IIterator::move_next failed: {:?}", e),
            }
        }
    }
}

// NOTE: We can't totally prevent iterator invalidation statically (because we cannot
//       prevent multiple ComPtr's pointing at the same object), but making the
//       mutating collection methods take `&mut self` already helps prevent many bugs.

impl<'a, T> IntoIterator for &'a IIterable<T> where T: RtType
{
    type Item = <T as RtType>::Out;
    type IntoIter = IteratorAdaptor<'a, T>;
    #[inline] fn into_iter(self) -> Self::IntoIter {
        IteratorAdaptor::new(self.first().unwrap().unwrap())
    }
}

impl<'a, T> IntoIterator for &'a IVector<T> where T: RtType, IIterable<T>: ComIid
{
    type Item = <T as RtType>::Out;
    type IntoIter = IteratorAdaptor<'a, T>;
    #[inline] fn into_iter(self) -> Self::IntoIter {
        IteratorAdaptor::new(self.query_interface::<IIterable<T>>().unwrap().first().unwrap().unwrap())
    }
}

impl<'a, T> IntoIterator for &'a IVectorView<T> where T: RtType, IIterable<T>: ComIid
{
    type Item = <T as RtType>::Out;
    type IntoIter = IteratorAdaptor<'a, T>;
    #[inline] fn into_iter(self) -> Self::IntoIter {
        IteratorAdaptor::new(self.query_interface::<IIterable<T>>().unwrap().first().unwrap().unwrap())
    }
}

impl<'a, T> IntoIterator for &'a IObservableVector<T> where T: RtType, IIterable<T>: ComIid
{
    type Item = <T as RtType>::Out;
    type IntoIter = IteratorAdaptor<'a, T>;
    #[inline] fn into_iter(self) -> Self::IntoIter {
        IteratorAdaptor::new(self.query_interface::<IIterable<T>>().unwrap().first().unwrap().unwrap())
    }
}

// TODO: Implement IntoIterator also for Map types

// TODO: also implement IndexMove for IVectorView etc once that exists (Index or IndexMut won't work since we can't return a reference)

macro_rules! RT_INTERFACE {
    ($(#[$attr:meta])* interface IInspectable $($rest:tt)*) => {
        RT_INTERFACE!($(#[$attr])* basic IInspectable $($rest)*);
        unsafe impl crate::RtInterface for IInspectable {}
        // no RtClassInterface for IInspectable!
    };

    ($(#[$attr:meta])* interface $interface:ident<$t1:ident, $t2:ident> $($rest:tt)*) => {
        RT_INTERFACE!($(#[$attr])* basic $interface<$t1,$t2> $($rest)*);
        unsafe impl<$t1: RtType, $t2: RtType> crate::RtInterface for $interface<$t1,$t2> {}
        unsafe impl<$t1: RtType, $t2: RtType> crate::RtClassInterface for $interface<$t1,$t2> {}
    };

    ($(#[$attr:meta])* interface $interface:ident<$t1:ident> $($rest:tt)*) => {
        RT_INTERFACE!($(#[$attr])* basic $interface<$t1> $($rest)*);
        unsafe impl<$t1: RtType> crate::RtInterface for $interface<$t1> {}
        unsafe impl<$t1: RtType> crate::RtClassInterface for $interface<$t1> {}
    };

    ($(#[$attr:meta])* interface $interface:ident $($rest:tt)*) => {
        RT_INTERFACE!($(#[$attr])* basic $interface $($rest)*);
        unsafe impl crate::RtInterface for $interface {}
        unsafe impl crate::RtClassInterface for $interface {}
    };

    ($(#[$attr:meta])* static interface $interface:ident<$t1:ident, $t2:ident> $($rest:tt)*) => {
        RT_INTERFACE!($(#[$attr])* basic $interface<$t1,$t2> $($rest)*);
        unsafe impl<$t1: RtType, $t2: RtType> crate::RtInterface for $interface<$t1,$t2> {}
    };

    ($(#[$attr:meta])* static interface $interface:ident<$t1:ident> $($rest:tt)*) => {
        RT_INTERFACE!($(#[$attr])* basic $interface<$t1> $($rest)*);
        unsafe impl<$t1: RtType> crate::RtInterface for $interface<$t1> {}
    };

    ($(#[$attr:meta])* static interface $interface:ident $($rest:tt)*) => {
        RT_INTERFACE!($(#[$attr])* basic $interface $($rest)*);
        unsafe impl crate::RtInterface for $interface {}
    };

    // version with no methods
    ($(#[$attr:meta])* basic $interface:ident ($vtbl:ident, $abiname:ident) : $pinterface:ident ($pvtbl:ty) [$iid:ident]
        {}
    ) => {
        #[repr(transparent)] #[allow(missing_copy_implementations)] #[doc(hidden)]
        pub struct $vtbl {
            pub parent: $pvtbl
        }
        $(#[$attr])* #[repr(transparent)] #[allow(missing_copy_implementations)] #[allow(non_camel_case_types)]
        pub struct $abiname {
            lpVtbl: *const $vtbl
        }
        $(#[$attr])* #[repr(transparent)] #[derive(Clone)]
        pub struct $interface(ComPtr<$abiname>);
        impl ComIid for $interface {
            #[inline] fn iid() -> &'static crate::Guid { &$iid }
        }
        impl crate::ComInterfaceAbi for $abiname {
            type Vtbl = $vtbl;
        }
        impl ComInterface for $interface {
            type Vtbl = $vtbl;
            type TAbi = $abiname;
            unsafe fn wrap_com(ptr: *mut Self::TAbi) -> Self { $interface(ComPtr::wrap_nonnull(ptr)) }
            fn get_abi(&self) -> &Self::TAbi { self.0.as_abi() }
        }
        impl crate::RtType for $interface {
            type In = Self;
            type Abi = *mut $abiname;
            type Out = Option<Self>;

            #[doc(hidden)] #[inline] unsafe fn unwrap(v: &Self::In) -> Self::Abi { v.0.as_abi() as *const _ as *mut _ }
            #[doc(hidden)] #[inline] unsafe fn uninitialized() -> Self::Abi { std::ptr::null_mut() }
            #[doc(hidden)] #[inline] unsafe fn wrap(v: Self::Abi) -> Self::Out {
                if v.is_null() {
                    None
                } else {
                    Some($interface(ComPtr::wrap_nonnull(v)))
                }
            }
            #[doc(hidden)] #[inline] unsafe fn wrap_nonnull(v: Self::Abi) -> Self { $interface(ComPtr::wrap_nonnull(v)) }
        }
        impl std::ops::Deref for $interface {
            type Target = $crate::$pinterface;
            #[inline]
            fn deref(&self) -> &$crate::$pinterface {
                unsafe { std::mem::transmute(self) }
            }
        }
        impl std::ops::DerefMut for $interface {
            #[inline]
            fn deref_mut(&mut self) -> &mut $crate::$pinterface {
                unsafe { std::mem::transmute(self) }
            }
        }
    };

    // version with methods, but without generic parameters
    ($(#[$attr:meta])* basic $interface:ident ($vtbl:ident, $abiname:ident) : $pinterface:ident ($pvtbl:ty) [$iid:ident]
        {$(
            $(#[cfg($cond_attr:meta)])* fn $method:ident(&self $(,$p:ident : $t:ty)*) -> $rtr:ty
        ),+}
    ) => {
        #[repr(C)] #[allow(missing_copy_implementations)] #[doc(hidden)]
        pub struct $vtbl {
            pub parent: $pvtbl
            $(, $(#[cfg($cond_attr)])* pub $method: unsafe extern "system" fn(
                This: *mut $interface
                $(,$p: $t)*
            ) -> $rtr)+
        }
        $(#[$attr])* #[repr(transparent)] #[allow(missing_copy_implementations)] #[allow(non_camel_case_types)]
        pub struct $abiname {
            lpVtbl: *const $vtbl
        }
        $(#[$attr])* #[repr(transparent)] #[derive(Clone)]
        pub struct $interface(ComPtr<$abiname>);
        impl ComIid for $interface {
            #[inline] fn iid() -> &'static crate::Guid { &$iid }
        }
        impl crate::ComInterfaceAbi for $abiname {
            type Vtbl = $vtbl;
        }
        impl ComInterface for $interface {
            type Vtbl = $vtbl;
            type TAbi = $abiname;
            unsafe fn wrap_com(ptr: *mut Self::TAbi) -> Self { $interface(ComPtr::wrap_nonnull(ptr)) }
            fn get_abi(&self) -> &Self::TAbi { self.0.as_abi() }
        }
        impl crate::RtType for $interface {
            type In = Self;
            type Abi = *mut $abiname;
            type Out = Option<Self>;

            #[doc(hidden)] #[inline] unsafe fn unwrap(v: &Self::In) -> Self::Abi { v.0.as_abi() as *const _ as *mut _ }
            #[doc(hidden)] #[inline] unsafe fn uninitialized() -> Self::Abi { std::ptr::null_mut() }
            #[doc(hidden)] #[inline] unsafe fn wrap(v: Self::Abi) -> Self::Out {
                if v.is_null() {
                    None
                } else {
                    Some($interface(ComPtr::wrap_nonnull(v)))
                }
            }
            #[doc(hidden)] #[inline] unsafe fn wrap_nonnull(v: Self::Abi) -> Self { $interface(ComPtr::wrap_nonnull(v)) }
        }
        impl std::ops::Deref for $interface {
            type Target = $crate::$pinterface;
            #[inline]
            fn deref(&self) -> &$crate::$pinterface {
                unsafe { std::mem::transmute(self) }
            }
        }
        impl std::ops::DerefMut for $interface {
            #[inline]
            fn deref_mut(&mut self) -> &mut $crate::$pinterface {
                unsafe { std::mem::transmute(self) }
            }
        }
    };
    // The $iid is actually not necessary, because it refers to the uninstantiated version of the interface,
    // which is irrelevant at runtime (it is used to generate the IIDs of the parameterized interfaces).
    ($(#[$attr:meta])* basic $interface:ident<$t1:ident> ($vtbl:ident, $abiname:ident) : $pinterface:ident ($pvtbl:ty) [$iid:ident]
        {$(
            $(#[cfg($cond_attr:meta)])* fn $method:ident(&self $(,$p:ident : $t:ty)*) -> $rtr:ty
        ),+}
    ) => {
        #[repr(C)] #[allow(missing_copy_implementations)] #[doc(hidden)]
        pub struct $vtbl<$t1> where $t1: RtType {
            pub parent: $pvtbl
            $(, $(#[cfg($cond_attr)])* pub $method: unsafe extern "system" fn(
                This: *mut $interface<$t1>
                $(,$p: $t)*
            ) -> $rtr)+
        }
        $(#[$attr])* #[repr(transparent)] #[allow(missing_copy_implementations)] #[allow(non_camel_case_types)]
        pub struct $abiname<$t1> where $t1: RtType {
            lpVtbl: *const $vtbl<$t1>,
        }
        $(#[$attr])* #[repr(transparent)] #[derive(Clone)]
        pub struct $interface<$t1: RtType>(ComPtr<$abiname<$t1>>);
        impl<$t1> crate::ComInterfaceAbi for $abiname<$t1> where $t1: RtType {
            type Vtbl = $vtbl<$t1>;
        }
        impl<$t1> ComInterface for $interface<$t1> where $t1: RtType {
            type Vtbl = $vtbl<$t1>;
            type TAbi = $abiname<$t1>;
            unsafe fn wrap_com(ptr: *mut Self::TAbi) -> Self { $interface(ComPtr::wrap_nonnull(ptr)) }
            fn get_abi(&self) -> &Self::TAbi { self.0.as_abi() }
        }
        impl<$t1> crate::RtType for $interface<$t1> where $t1: RtType{
            type In = Self;
            type Abi = *mut $abiname<$t1>;
            type Out = Option<Self>;

            #[doc(hidden)] #[inline] unsafe fn unwrap(v: &Self::In) -> Self::Abi { v.0.as_abi() as *const _ as *mut _ }
            #[doc(hidden)] #[inline] unsafe fn uninitialized() -> Self::Abi { std::ptr::null_mut() }
            #[doc(hidden)] #[inline] unsafe fn wrap(v: Self::Abi) -> Self::Out {
                if v.is_null() {
                    None
                } else {
                    Some($interface(ComPtr::wrap_nonnull(v)))
                }
            }
            #[doc(hidden)] #[inline] unsafe fn wrap_nonnull(v: Self::Abi) -> Self { $interface(ComPtr::wrap_nonnull(v)) }
        }
        impl<$t1> std::ops::Deref for $interface<$t1> where $t1: RtType {
            type Target = $pinterface;
            #[inline]
            fn deref(&self) -> &$pinterface {
                unsafe { std::mem::transmute(self) }
            }
        }
        impl<$t1> std::ops::DerefMut for $interface<$t1> where $t1: RtType {
            #[inline]
            fn deref_mut(&mut self) -> &mut $pinterface {
                unsafe { std::mem::transmute(self) }
            }
        }
    };

    ($(#[$attr:meta])* basic $interface:ident<$t1:ident, $t2:ident> ($vtbl:ident, $abiname:ident) : $pinterface:ident ($pvtbl:ty) [$iid:ident]
        {$(
            $(#[cfg($cond_attr:meta)])* fn $method:ident(&self $(,$p:ident : $t:ty)*) -> $rtr:ty
        ),+}
    ) => {
        #[repr(C)] #[allow(missing_copy_implementations)] #[doc(hidden)]
        pub struct $vtbl<$t1, $t2> where $t1: RtType, $t2: RtType {
            pub parent: $pvtbl
            $(, $(#[cfg($cond_attr)])* pub $method: unsafe extern "system" fn(
                This: *mut $interface<$t1, $t2>
                $(,$p: $t)*
            ) -> $rtr)+
        }
        $(#[$attr])* #[repr(transparent)] #[allow(missing_copy_implementations)] #[allow(non_camel_case_types)]
        pub struct $abiname<$t1, $t2> where $t1: RtType, $t2: RtType {
            lpVtbl: *const $vtbl<$t1, $t2>,
        }
        $(#[$attr])* #[repr(transparent)] #[derive(Clone)]
        pub struct $interface<$t1: RtType, $t2: RtType>(ComPtr<$abiname<$t1, $t2>>);
        impl<$t1, $t2> crate::ComInterfaceAbi for $abiname<$t1, $t2> where $t1: RtType, $t2: RtType {
            type Vtbl = $vtbl<$t1, $t2>;
        }
        impl<$t1, $t2> ComInterface for $interface<$t1, $t2> where $t1: RtType, $t2: RtType {
            type Vtbl = $vtbl<$t1, $t2>;
            type TAbi = $abiname<$t1, $t2>;
            unsafe fn wrap_com(ptr: *mut Self::TAbi) -> Self { $interface(ComPtr::wrap_nonnull(ptr)) }
            fn get_abi(&self) -> &Self::TAbi { self.0.as_abi() }
        }
        impl<$t1, $t2> crate::RtType for $interface<$t1, $t2> where $t1: RtType, $t2: RtType {
            type In = Self;
            type Abi = *mut $abiname<$t1, $t2>;
            type Out = Option<Self>;

            #[doc(hidden)] #[inline] unsafe fn unwrap(v: &Self::In) -> Self::Abi { v.0.as_abi() as *const _ as *mut _ }
            #[doc(hidden)] #[inline] unsafe fn uninitialized() -> Self::Abi { std::ptr::null_mut() }
            #[doc(hidden)] #[inline] unsafe fn wrap(v: Self::Abi) -> Self::Out {
                if v.is_null() {
                    None
                } else {
                    Some($interface(ComPtr::wrap_nonnull(v)))
                }
            }
            #[doc(hidden)] #[inline] unsafe fn wrap_nonnull(v: Self::Abi) -> Self { $interface(ComPtr::wrap_nonnull(v)) }
        }
        impl<$t1, $t2> std::ops::Deref for $interface<$t1, $t2> where $t1: RtType, $t2: RtType {
            type Target = $pinterface;
            #[inline]
            fn deref(&self) -> &$pinterface {
                unsafe { std::mem::transmute(self) }
            }
        }
        impl<$t1, $t2> std::ops::DerefMut for $interface<$t1, $t2> where $t1: RtType, $t2: RtType {
            #[inline]
            fn deref_mut(&mut self) -> &mut $pinterface {
                unsafe { std::mem::transmute(self) }
            }
        }
    };
}

macro_rules! RT_DELEGATE {
    // without generic parameters
    (delegate $interface:ident ($vtbl:ident, $abiname:ident, $imp:ident) [$iid:ident] {
        $(#[cfg($cond_attr:meta)])* fn Invoke(&self $(,$p:ident : $t:ty)*) -> HRESULT
    }) => {
        RT_INTERFACE!{basic $interface($vtbl, $abiname) : IUnknown(IUnknownVtbl) [$iid] {
            $(#[cfg($cond_attr)])* fn Invoke(&self $(,$p : $t)*) -> HRESULT
        }}

        $(#[cfg($cond_attr)])*
        impl $interface {
            #[inline] pub fn new<_F_>(f: _F_) -> $interface
                where _F_: 'static + Send + FnMut($($t),*) -> Result<()>, $interface: ComIid {
                $imp::new(f).into_interface()
            }
        }

        $(#[cfg($cond_attr)])*
        struct $imp<_F_> where _F_: 'static + Send + FnMut($($t),*) -> Result<()> {
            invoke: _F_
        }

        $(#[cfg($cond_attr)])*
        impl<_F_> $imp<_F_>
            where $interface: ComIid, _F_: 'static + Send + FnMut($($t),*) -> Result<()>
        {
            #[inline]
            pub fn new(f: _F_) -> $imp<_F_> {
                $imp {
                    invoke: f,
                }
            }
        }

        $(#[cfg($cond_attr)])*
        impl<_F_> crate::rt::handler::ComClass<$interface> for $imp<_F_>
            where $interface: ComIid, _F_: 'static + Send + FnMut($($t),*) -> Result<()>
        {
            #[inline]
            fn get_vtbl() -> $vtbl {
                $vtbl {
                    parent: IUnknownVtbl {
                        QueryInterface: crate::rt::handler::ComReprHandler_QueryInterface::<$imp<_F_>, _>,
                        AddRef: crate::rt::handler::ComRepr_AddRef::<$imp<_F_>>,
                        Release: crate::rt::handler::ComRepr_Release::<$imp<_F_>>,
                    },
                    Invoke: {
                        unsafe extern "system" fn Invoke<_F_>(this_: *mut $interface $(,$p : $t)*) -> HRESULT
                            where $interface: ComIid, _F_: 'static + Send + FnMut($($t),*) -> Result<()>
                        {
                            let this: &mut $imp<_F_> = crate::rt::handler::ComClass::from_interface(this_);
                            match (this.invoke)($($p),*) {
                                Ok(()) => S_OK,
                                Err(err) => err.as_hresult()
                            }
                        }
                        Invoke::<_F_>
                    }
                }
            }
        }

        /*impl<_F_> Drop for $imp<_F_>
            where _F_: 'static + Send + FnMut($($t),*) -> HRESULT
        {
            fn drop(&mut self) {
                println!("DROPPED {}<...>!", stringify!($imp));
            }
        }*/
    };

    // with generic parameters
    (delegate $interface:ident<$($ht:ident),+> ($vtbl:ident, $abiname:ident, $imp:ident) [$iid:ident] {
        $(#[cfg($cond_attr:meta)])* fn Invoke(&self $(,$p:ident : $t:ty)*) -> HRESULT
    }) => {
        RT_INTERFACE!{basic $interface<$($ht),+>($vtbl, $abiname) : IUnknown(IUnknownVtbl) [$iid] {
            $(#[cfg($cond_attr)])* fn Invoke(&self $(,$p : $t)*) -> HRESULT
        }}

        impl<$($ht: RtType + 'static),+> $interface<$($ht),+> {
            #[inline] pub fn new<_F_>(f: _F_) -> $interface<$($ht),+>
                where _F_: 'static + Send + FnMut($($t),*) -> Result<()>, $interface<$($ht),+>: ComIid {
                $imp::new(f).into_interface()
            }
        }

        struct $imp<$($ht: RtType),+ , _F_> where _F_: 'static + Send + FnMut($($t),*) -> Result<()> {
            invoke: _F_,
            phantom: std::marker::PhantomData<($($ht),+)>
        }

        impl<$($ht: RtType + 'static),+ , _F_> $imp<$($ht),+ , _F_>
            where $interface<$($ht),+>: ComIid, _F_: 'static + Send + FnMut($($t),*) -> Result<()>
        {
            #[inline]
            pub fn new(f: _F_) -> $imp<$($ht),+ , _F_> {
                $imp {
                    invoke: f,
                    phantom: std::marker::PhantomData
                }
            }
        }

        impl<$($ht: RtType + 'static),+ , _F_> crate::rt::handler::ComClass<$interface<$($ht),+>> for $imp<$($ht),+ , _F_>
            where $interface<$($ht),+>: ComIid, _F_: 'static + Send + FnMut($($t),*) -> Result<()>
        {
            #[inline]
            fn get_vtbl() -> $vtbl<$($ht),+> {
                $vtbl::<$($ht),+> {
                    parent: IUnknownVtbl {
                        QueryInterface: crate::rt::handler::ComReprHandler_QueryInterface::<$imp<$($ht),+ , _F_>, _>,
                        AddRef: crate::rt::handler::ComRepr_AddRef::<$imp<$($ht),+ , _F_>>,
                        Release: crate::rt::handler::ComRepr_Release::<$imp<$($ht),+ , _F_>>,
                    },
                    Invoke: {
                        unsafe extern "system" fn Invoke<$($ht: RtType + 'static),+ , _F_>(this_: *mut $interface<$($ht),+> $(,$p : $t)*) -> HRESULT
                            where $interface<$($ht),+>: ComIid, _F_: 'static + Send + FnMut($($t),*) -> Result<()>
                        {
                            let this: &mut $imp<$($ht),+ , _F_> = crate::rt::handler::ComClass::from_interface(this_);
                            match (this.invoke)($($p),*) {
                                Ok(()) => S_OK,
                                Err(err) => err.as_hresult()
                            }
                        }
                        Invoke::<$($ht),+ , _F_>
                    }
                }
            }
        }

        /*impl<$($ht: RtType),+ , _F_> Drop for $imp<$($ht),+ , _F_>
            where _F_: 'static + Send + FnMut($($t),*) -> HRESULT
        {
            fn drop(&mut self) {
                println!("DROPPED {}<...>!", stringify!($imp));
            }
        }*/
    };
}

macro_rules! RT_CLASS {
    {static class $cls:ident} => {
        pub struct $cls; // does not exist at runtime and has no ABI representation
    };

    {class $cls:ident : $interface:ty} => {
        #[repr(transparent)] #[derive(Clone)]
        pub struct $cls(ComPtr<<$interface as ComInterface>::TAbi>);
        unsafe impl crate::RtInterface for $cls {}
        unsafe impl crate::RtClassInterface for $cls {}
        impl ComInterface for $cls {
            type Vtbl = <$interface as ComInterface>::Vtbl;
            type TAbi = <$interface as ComInterface>::TAbi;
            unsafe fn wrap_com(ptr: *mut Self::TAbi) -> Self { $cls(ComPtr::wrap_nonnull(ptr)) }
            fn get_abi(&self) -> &Self::TAbi { self.0.as_abi() }
        }
        impl ComIid for $cls {
            #[inline] fn iid() -> &'static crate::Guid { <$interface as ComIid>::iid() }
        }
        impl crate::RtType for $cls {
            type In = Self;
            type Abi = <$interface as RtType>::Abi;
            type Out = Option<Self>;
            
            #[doc(hidden)] #[inline] unsafe fn unwrap(v: &Self::In) -> Self::Abi { v.0.as_abi() as *const _ as *mut _ }
            #[doc(hidden)] #[inline] unsafe fn uninitialized() -> Self::Abi { std::ptr::null_mut() }
            #[doc(hidden)] #[inline] unsafe fn wrap(v: Self::Abi) -> Self::Out {
                if v.is_null() {
                    None
                } else {
                    Some($cls(ComPtr::wrap_nonnull(v)))
                }
            }
            #[doc(hidden)] #[inline] unsafe fn wrap_nonnull(v: Self::Abi) -> Self { $cls(ComPtr::wrap_nonnull(v)) }
        }
        impl std::ops::Deref for $cls {
            type Target = $interface;
            #[inline]
            fn deref(&self) -> &$interface {
                unsafe { &*(self as *const _ as *const Self::Target) }
            }
        }
        impl std::ops::DerefMut for $cls {
            #[inline]
            fn deref_mut(&mut self) -> &mut $interface {
                unsafe { &mut *(self as *mut _ as *mut Self::Target) }
            }
        }
    };
}

macro_rules! DEFINE_CLSID {
    ($clsname:ident($id:expr) [$idname:ident]) => {
        const $idname: &'static [u16] = $id; // Full name of the class as null-terminated UTF16 string
        impl crate::RtNamedClass for $clsname {
            #[inline]
            fn name() -> &'static [u16] { $idname } 
        }
    }
}

macro_rules! RT_ENUM {
    {enum $name:ident : $t:ty { $($variant:ident = $value:expr,)+ }} => {
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        #[allow(non_upper_case_globals)]
        pub struct $name(pub $t);

        impl $name {
            $(pub const $variant: $name = $name($value);)+
        }
        unsafe impl crate::RtValueType for $name {}
    };
}

macro_rules! RT_STRUCT {
    { struct $name:ident { $($field:ident: $ftype:ty,)* }} => {
        #[repr(C)] #[derive(Debug,Copy,Clone)]
        pub struct $name {
            $(pub $field: $ftype,)*
        }
        unsafe impl crate::RtValueType for $name {}
    };
}

macro_rules! RT_PINTERFACE {
    (
        for $t:ty => [$l:expr, $w1:expr, $w2:expr, $b1:expr, $b2:expr, $b3:expr, $b4:expr, $b5:expr,
        $b6:expr, $b7:expr, $b8:expr] as $iid:ident
    ) => {
        DEFINE_IID!($iid, $l,$w1,$w2,$b1,$b2,$b3,$b4,$b5,$b6,$b7,$b8);
        impl ComIid for $t {
            #[inline] fn iid() -> &'static crate::Guid { &$iid }
        }
    };
}

pub(crate) mod handler;
pub(crate) mod async_util;
pub(crate) mod gen; // import auto-generated definitions (has to come after macro definitions)

// FIXME: maybe better reexport from winapi?
DEFINE_IID!(IID_IInspectable, 0xAF86E2E0, 0xB12D, 0x4c6a, 0x9C, 0x5A, 0xD7, 0xAA, 0x65, 0x10, 0x1E, 0x90);
RT_INTERFACE!{
/// The `IInspectable` interface is the base interface for all Windows Runtime classes.
interface IInspectable(IInspectableVtbl, IInspectable_Abi): IUnknown(IUnknownVtbl) [IID_IInspectable]  {
    fn GetIids(&self, iidCount: *mut ULONG, iids: *mut *mut IID) -> HRESULT,
    fn GetRuntimeClassName(&self, className: *mut HSTRING) -> HRESULT,
    fn GetTrustLevel(&self, trustLevel: *mut crate::TrustLevel) -> HRESULT
}}
impl IInspectable {
    /// Returns the interfaces that are implemented by the current Windows Runtime object.
    #[inline]
    pub fn get_iids(&self) -> ComArray<Guid> {
        let mut result = std::ptr::null_mut();
        let mut count = 0;
        let hr = unsafe { ((*self.get_abi().lpVtbl).GetIids)(self.get_abi() as *const _ as *mut _, &mut count, &mut result) };
        assert_eq!(hr, S_OK);
        let result = result as *mut Guid; // convert from w::GUID to (binary compatible) Guid
        unsafe { ComArray::from_raw(count, result) }
    }

    /// Returns the trust level of the current Windows Runtime object.
    #[inline]
    pub fn get_trust_level(&self) -> crate::TrustLevel {
        let mut result = unsafe { std::mem::zeroed() };
        let hr = unsafe { ((*self.get_abi().lpVtbl).GetTrustLevel)(self.get_abi() as *const _ as *mut _, &mut result) };
        assert_eq!(hr, S_OK);
        result
    }

    /// Publically exposed via RtClassInterface
    pub(crate) fn get_runtime_class_name(&self) -> HString {
        let mut result = std::ptr::null_mut();
        let hr = unsafe { ((*self.get_abi().lpVtbl).GetRuntimeClassName)(self.get_abi() as *const _ as *mut _, &mut result) };
        assert_eq!(hr, S_OK);
        unsafe { HString::wrap(result) }
    }
}

DEFINE_IID!(IID_IActivationFactory, 0x00000035, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46);
RT_INTERFACE!{
/// Enables classes to be activated using a default constructor.
/// This interface should not be used directly, but the `new()` method of
/// the `RtDefaultConstructible` trait should be used instead.
interface IActivationFactory(IActivationFactoryVtbl, IActivationFactory_Abi): IInspectable(IInspectableVtbl) [IID_IActivationFactory]  {
    fn ActivateInstance(&self, instance: *mut <IInspectable as RtType>::Abi) -> HRESULT
}}

impl IActivationFactory {
    #[inline]
    pub fn activate_instance(&self) -> IInspectable {
        let mut result = std::ptr::null_mut();
        let hr = unsafe { ((*self.get_abi().lpVtbl).ActivateInstance)(self as *const _ as *mut _, &mut result) };
        assert_eq!(hr, S_OK);
        unsafe { IInspectable::wrap_com(result) }
    }
}

DEFINE_IID!(IID_IMemoryBufferByteAccess, 0x5b0d3235, 0x4dba, 0x4d44, 0x86, 0x5e, 0x8f, 0x1d, 0x0e, 0x4f, 0xd0, 0x4d);
RT_INTERFACE!{
/// Provides direct byte access to the memory buffer underlying an `IMemoryBuffer`.
interface IMemoryBufferByteAccess(IMemoryBufferByteAccessVtbl, IMemoryBufferByteAccess_Abi) : IUnknown(IUnknownVtbl) [IID_IMemoryBufferByteAccess]  {
    fn GetBuffer(&self, value: *mut *mut u8, capacity: *mut u32) -> HRESULT
}}

impl IMemoryBufferByteAccess {
    /// Provides direct byte access to the memory buffer underlying an `IMemoryBuffer`.
    /// To use `IMemoryBufferByteAccess`, you first need to obtain an `IMemoryBufferReference`, then
    /// call `query_interface::<IMemoryBufferByteAccess>()` on that.
    ///
    /// This method is marked `unsafe`, because the buffer might be invalidated during its lifetime.
    /// Due to the nature of COM objects, the buffer might be referenced and modified or closed
    /// by some other instance, which Rust's type system cannot know about. The `Close` event on
    /// `IMemoryBufferReference` can be used to be notified when the buffer is closed.
    ///
    /// An empty slice is returned if the underlying buffer has already been closed.
    #[inline]
    pub unsafe fn get_buffer(&self) -> &[u8] {
        let mut ptr = std::ptr::null_mut();
        let mut capacity: u32 = 0;
        let hr = ((*self.get_abi().lpVtbl).GetBuffer)(self as *const _ as *mut _, &mut ptr, &mut capacity);
        assert_eq!(hr, S_OK);
        if capacity == 0 {
            ptr = 1 as *mut u8; // null pointer is not allowed
        }
        std::slice::from_raw_parts(ptr, capacity as usize)
    }
}

/// Determines the concurrency model used for incoming calls to the objects created by a thread
/// that was initialized with a given apartment type (see also `init_apartment`).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ApartmentType {
    /// Initializes the thread in the multi-threaded apartment (MTA).
    MTA = RO_INIT_MULTITHREADED,
    
    /// Initializes the thread as a single-threaded apartment (STA).
    STA = RO_INIT_SINGLETHREADED
}

/// Initializes the current thread for use with the Windows Runtime. This is usually not needed,
/// because winrt-rust ensures that threads are implicitly assigned to the multi-threaded apartment (MTA).
/// However, if you need your thread to be initialized as a single-threaded apartment (STA), you can
/// call `init_apartment(ApartmentType::STA)`. Only call this when you own the thread!
pub fn init_apartment(apartment_type: ApartmentType) {
    let hr = unsafe { RoInitialize(apartment_type as u32) };
    assert!(hr == S_OK || hr == S_FALSE, "failed to call RoInitialize: error {}", hr);
}

/// Uninitializes the Windows Runtime in the current thread. This is usually not
/// needed, because uninitialization happens automatically on process termination.
/// Make sure that you never call this from a thread that still has references to
/// Windows Runtime objects.
pub fn uninit_apartment() {
    unsafe { RoUninitialize() };
}
