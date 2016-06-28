use std::ptr;

use super::{ComInterface, HString, HStringRef, ComPtr, ComIid, Guid};

use w::{HRESULT, HSTRING, S_OK, ULONG, BOOL, TRUE, FALSE, TrustLevel, IID};

/// This means that the interfaced is based on IInspectable
pub unsafe trait RtInterface: ComInterface {}

pub unsafe trait RtValueType {}

unsafe impl RtValueType for f32 {}
unsafe impl RtValueType for f64 {}
unsafe impl RtValueType for i16 {}
unsafe impl RtValueType for i32 {}
unsafe impl RtValueType for i64 {}
unsafe impl RtValueType for u8 {}
unsafe impl RtValueType for u16 {}
unsafe impl RtValueType for u32 {}
unsafe impl RtValueType for u64 {}

/// This is a trait implemented by all types that can be used as generic parameters of parameterized interfaces
pub trait RtType {
    type In;
    type Abi;
    type Out;

    unsafe fn unwrap(input: Self::In) -> Self::Abi;
    unsafe fn uninitialized() -> Self::Abi;
    unsafe fn wrap(abi: Self::Abi) -> Self::Out;
}

impl<'a> RtType for &'a str {
    type In = HStringRef<'a>;
    type Abi = ::w::HSTRING;
    type Out = HString;

    unsafe fn unwrap(v: Self::In) -> Self::Abi {
        v.get()
    }
    unsafe fn uninitialized() -> Self::Abi {
        ::std::ptr::null_mut()
    }
    unsafe fn wrap(v: Self::Abi) -> Self::Out {
        HString::wrap(v)
    }
}

impl<'a> RtType for bool {
    type In = bool;
    type Abi = ::w::BOOL;
    type Out = bool;

    unsafe fn unwrap(v: Self::In) -> Self::Abi {
        if v {
            ::w::TRUE
        } else {
            ::w::FALSE
        }
    }
    unsafe fn uninitialized() -> Self::Abi {
        ::w::FALSE
    }
    unsafe fn wrap(v: Self::Abi) -> Self::Out {
        v == ::w::TRUE
    }
}

impl RtType for Guid {
    type In = Guid;
    type Abi = ::w::GUID; // could also directly use Guid, since they are binary-compatible
    type Out = Guid;

    unsafe fn unwrap(v: Self::In) -> Self::Abi {
        v.as_iid()
    }
    unsafe fn uninitialized() -> Self::Abi {
        ::std::mem::zeroed()
    }
    unsafe fn wrap(v: Self::Abi) -> Self::Out {
        Guid::from(v)
    }
}

impl<T> RtType for T where T: RtValueType
{
    type In = T;
    type Abi = T;
    type Out = T;

    unsafe fn unwrap(v: Self::In) -> Self::Abi {
        v
    }
    unsafe fn uninitialized() -> Self::Abi {
        ::std::mem::zeroed()
    }
    unsafe fn wrap(v: Self::Abi) -> Self::Out {
        v
    }
}

// We can also implement IntoIterator for IIterable<T> and IVectorView<T> and references to those
// TODO: This should be extended to more types (at least IVector, IMap, IMapView, IObservableVector, IObservableMap)
impl<T> IntoIterator for ComPtr<IIterable<T>> where T: RtType
{
    type Item = <T as RtType>::Out;
    type IntoIter = ComPtr<IIterator<T>>;
    fn into_iter(mut self) -> Self::IntoIter {
        unsafe {
            let mut iterator = ptr::null_mut();
            assert_eq!(self.First(&mut iterator), S_OK);
            ComPtr::wrap(iterator)
        }
    }
}

impl<'a, T> IntoIterator for &'a mut ComPtr<IIterable<T>> where T: RtType
{
    type Item = <T as RtType>::Out;
    type IntoIter = ComPtr<IIterator<T>>;
    fn into_iter(mut self) -> Self::IntoIter {
        unsafe {
            let mut iterator = ptr::null_mut();
            assert_eq!(self.First(&mut iterator), S_OK);
            ComPtr::wrap(iterator)
        }
    }
}

impl<T> IntoIterator for ComPtr<IVectorView<T>> where T: RtType, IIterable<T>: ComIid
{
    type Item = <T as RtType>::Out;
    type IntoIter = ComPtr<IIterator<T>>;
    fn into_iter(self) -> Self::IntoIter {
        self.query_interface::<IIterable<T>>().unwrap().into_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut ComPtr<IVectorView<T>> where T: RtType, IIterable<T>: ComIid
{
    type Item = <T as RtType>::Out;
    type IntoIter = ComPtr<IIterator<T>>;
    fn into_iter(self) -> Self::IntoIter {
        self.query_interface::<IIterable<T>>().unwrap().into_iter()
    }
}

// TODO: also implement IndexMove for IVectorView etc once that exists (Index or IndexMut won't work since we can't return a reference)

impl<T> Iterator for ComPtr<IIterator<T>> where T: RtType
{
    type Item = <T as RtType>::Out;

    // TODO: This could potentially be made faster by using the output of MoveNext instead of calling HasCurrent
    //       in every iteration. That would require a wrapper struct with a boolean flag.
    fn next(&mut self) -> Option<Self::Item> {
        let has_next = unsafe {
            let mut hasCurrent: BOOL = FALSE;
            self.get_HasCurrent(&mut hasCurrent);
            hasCurrent == TRUE
        };
        if has_next {
            unsafe {
                let mut current: <T as RtType>::Abi = <T as RtType>::uninitialized();
                assert_eq!(self.get_Current(&mut current), S_OK);
                let mut hasCurrent: BOOL = FALSE;
                assert_eq!(self.MoveNext(&mut hasCurrent), S_OK);
                Some(<T as RtType>::wrap(current))
            }
        } else {
            None
        }
    }
}

macro_rules! RT_INTERFACE {
// Specialized version for IUnknown (does not impl RtInterface) -> TODO: get rid of this
    (interface $interface:ident ($vtbl:ident) : IUnknown(IUnknownVtbl) [$iid:ident]
        {$(
            fn $method:ident(&mut self $(,$p:ident : $t:ty)*) -> $rtr:ty
        ),+}
    ) => {
        #[repr(C)] #[allow(missing_copy_implementations)] #[doc(hidden)]
        pub struct $vtbl {
            pub parent: $crate::IUnknownVtbl
            $(,pub $method: unsafe extern "system" fn(
                This: *mut $interface
                $(,$p: $t)*
            ) -> $rtr)+
        }
        #[repr(C)] #[derive(Debug)] #[allow(missing_copy_implementations)]
        pub struct $interface {
            lpVtbl: *const $vtbl
        }
        impl $interface {
            #[inline]
            $(pub unsafe fn $method(&mut self $(,$p: $t)*) -> $rtr {
                ((*self.lpVtbl).$method)(self $(,$p)*)
            })+
        }
        impl ComIid for $interface {
            fn iid() -> &'static ::Guid { &$iid }
        }
        impl ComInterface for $interface {
            type Vtbl = $vtbl;
        }
        impl<'a> RtType for &'a $interface {
            type In = &'a mut $interface;
            type Abi = *mut $interface;
            type Out = ComPtr<$interface>;

            unsafe fn unwrap(v: Self::In) -> Self::Abi { v }
            unsafe fn uninitialized() -> Self::Abi { ::std::ptr::null_mut() }
            unsafe fn wrap(v: Self::Abi) -> Self::Out { ComPtr::wrap(v) }
        }

        impl ::std::ops::Deref for $interface {
            type Target = $crate::IUnknown;
            #[inline]
            fn deref(&self) -> &$crate::IUnknown {
                unsafe { ::std::mem::transmute(self) }
            }
        }
        impl ::std::ops::DerefMut for $interface {
            #[inline]
            fn deref_mut(&mut self) -> &mut $crate::IUnknown {
                unsafe { ::std::mem::transmute(self) }
            }
        }
    };

    (interface $interface:ident<$t1:ident> ($vtbl:ident) : IUnknown(IUnknownVtbl) [$iid:ident]
        {$(
            fn $method:ident(&mut self $(,$p:ident : $t:ty)*) -> $rtr:ty
        ),+}
    ) => {
        #[repr(C)] #[allow(missing_copy_implementations)] #[doc(hidden)]
        pub struct $vtbl<$t1> where $t1: RtType {
            pub parent: $crate::IUnknownVtbl
            $(,pub $method: unsafe extern "system" fn(
                This: *mut $interface<$t1>
                $(,$p: $t)*
            ) -> $rtr)+
        }
        #[repr(C)] #[derive(Debug)] #[allow(missing_copy_implementations)]
        pub struct $interface<$t1> where $t1: RtType {
            lpVtbl: *const $vtbl<$t1>,
        }
        impl<$t1> $interface<$t1> where $t1: RtType {
            #[inline]
            $(pub unsafe fn $method(&mut self $(,$p: $t)*) -> $rtr {
                ((*self.lpVtbl).$method)(self $(,$p)*)
            })+
        }
        impl<$t1: 'static> ComInterface for $interface<$t1> where $t1: RtType {
            type Vtbl = $vtbl<$t1>;
        }
        impl<'a, $t1> RtType for &'a $interface<$t1> where $t1: RtType{
            type In = &'a mut $interface<$t1>;
            type Abi = *mut $interface<$t1>;
            type Out = ComPtr<$interface<$t1>>;

            unsafe fn unwrap(v: Self::In) -> Self::Abi { v }
            unsafe fn uninitialized() -> Self::Abi { ::std::ptr::null_mut() }
            unsafe fn wrap(v: Self::Abi) -> Self::Out { ComPtr::wrap(v) }
        }
        impl<$t1> ::std::ops::Deref for $interface<$t1> where $t1: RtType {
            type Target = $crate::IUnknown;
            #[inline]
            fn deref(&self) -> &$crate::IUnknown {
                unsafe { ::std::mem::transmute(self) }
            }
        }
        impl<$t1> ::std::ops::DerefMut for $interface<$t1> where $t1: RtType {
            #[inline]
            fn deref_mut(&mut self) -> &mut $crate::IUnknown {
                unsafe { ::std::mem::transmute(self) }
            }
        }
    };

    (interface $interface:ident ($vtbl:ident) : $pinterface:ident ($pvtbl:ident) [$iid:ident]
        {}
    ) => {
        #[repr(C)] #[allow(missing_copy_implementations)] #[doc(hidden)]
        pub struct $vtbl {
            pub parent: $crate::$pvtbl
        }
        #[repr(C)] #[derive(Debug)] #[allow(missing_copy_implementations)]
        pub struct $interface {
            lpVtbl: *const $vtbl
        }
        impl ComIid for $interface {
            fn iid() -> &'static ::Guid { &$iid }
        }
        impl ComInterface for $interface {
            type Vtbl = $vtbl;
        }
        unsafe impl RtInterface for $interface {}
        impl<'a> RtType for &'a $interface {
            type In = &'a mut $interface;
            type Abi = *mut $interface;
            type Out = ComPtr<$interface>;

            unsafe fn unwrap(v: Self::In) -> Self::Abi { v }
            unsafe fn uninitialized() -> Self::Abi { ::std::ptr::null_mut() }
            unsafe fn wrap(v: Self::Abi) -> Self::Out { ComPtr::wrap(v) }
        }
        impl ::std::ops::Deref for $interface {
            type Target = $crate::$pinterface;
            #[inline]
            fn deref(&self) -> &$crate::$pinterface {
                unsafe { ::std::mem::transmute(self) }
            }
        }
        impl ::std::ops::DerefMut for $interface {
            #[inline]
            fn deref_mut(&mut self) -> &mut $crate::$pinterface {
                unsafe { ::std::mem::transmute(self) }
            }
        }
    };

    (interface $interface:ident ($vtbl:ident) : $pinterface:ident ($pvtbl:ident) [$iid:ident]
        {$(
            fn $method:ident(&mut self $(,$p:ident : $t:ty)*) -> $rtr:ty
        ),+}
    ) => {
        #[repr(C)] #[allow(missing_copy_implementations)] #[doc(hidden)]
        pub struct $vtbl {
            pub parent: $crate::$pvtbl
            $(,pub $method: unsafe extern "system" fn(
                This: *mut $interface
                $(,$p: $t)*
            ) -> $rtr)+
        }
        #[repr(C)] #[derive(Debug)] #[allow(missing_copy_implementations)]
        pub struct $interface {
            lpVtbl: *const $vtbl
        }
        impl $interface {
            #[inline]
            $(pub unsafe fn $method(&mut self $(,$p: $t)*) -> $rtr {
                ((*self.lpVtbl).$method)(self $(,$p)*)
            })+
        }
        impl ComIid for $interface {
            fn iid() -> &'static ::Guid { &$iid }
        }
        impl ComInterface for $interface {
            type Vtbl = $vtbl;
        }
        unsafe impl RtInterface for $interface {}
        impl<'a> RtType for &'a $interface {
            type In = &'a mut $interface;
            type Abi = *mut $interface;
            type Out = ComPtr<$interface>;

            unsafe fn unwrap(v: Self::In) -> Self::Abi { v }
            unsafe fn uninitialized() -> Self::Abi { ::std::ptr::null_mut() }
            unsafe fn wrap(v: Self::Abi) -> Self::Out { ComPtr::wrap(v) }
        }
        impl ::std::ops::Deref for $interface {
            type Target = $crate::$pinterface;
            #[inline]
            fn deref(&self) -> &$crate::$pinterface {
                unsafe { ::std::mem::transmute(self) }
            }
        }
        impl ::std::ops::DerefMut for $interface {
            #[inline]
            fn deref_mut(&mut self) -> &mut $crate::$pinterface {
                unsafe { ::std::mem::transmute(self) }
            }
        }
    };
    // The $iid is actually not necessary, because it refers to the uninstantiated version of the interface,
    // which is irrelevant at runtime (it is used to generate the IIDs of the parameterized interfaces).
    (interface $interface:ident<$t1:ident> ($vtbl:ident) : $pinterface:ident ($pvtbl:ident) [$iid:ident]
        {$(
            fn $method:ident(&mut self $(,$p:ident : $t:ty)*) -> $rtr:ty
        ),+}
    ) => {
        #[repr(C)] #[allow(missing_copy_implementations)] #[doc(hidden)]
        pub struct $vtbl<$t1> where $t1: RtType {
            pub parent: $crate::$pvtbl
            $(,pub $method: unsafe extern "system" fn(
                This: *mut $interface<$t1>
                $(,$p: $t)*
            ) -> $rtr)+
        }
        #[repr(C)] #[derive(Debug)] #[allow(missing_copy_implementations)]
        pub struct $interface<$t1> where $t1: RtType {
            lpVtbl: *const $vtbl<$t1>,
        }
        impl<$t1> $interface<$t1> where $t1: RtType {
            #[inline]
            $(pub unsafe fn $method(&mut self $(,$p: $t)*) -> $rtr {
                ((*self.lpVtbl).$method)(self $(,$p)*)
            })+
        }
        impl<$t1: 'static> ComInterface for $interface<$t1> where $t1: RtType {
            type Vtbl = $vtbl<$t1>;
        }
        unsafe impl<$t1: 'static> RtInterface for $interface<$t1> where $t1: RtType {}
        impl<'a, $t1> RtType for &'a $interface<$t1> where $t1: RtType{
            type In = &'a mut $interface<$t1>;
            type Abi = *mut $interface<$t1>;
            type Out = ComPtr<$interface<$t1>>;

            unsafe fn unwrap(v: Self::In) -> Self::Abi { v }
            unsafe fn uninitialized() -> Self::Abi { ::std::ptr::null_mut() }
            unsafe fn wrap(v: Self::Abi) -> Self::Out { ComPtr::wrap(v) }
        }
        impl<$t1> ::std::ops::Deref for $interface<$t1> where $t1: RtType {
            type Target = $pinterface;
            #[inline]
            fn deref(&self) -> &$pinterface {
                unsafe { ::std::mem::transmute(self) }
            }
        }
        impl<$t1> ::std::ops::DerefMut for $interface<$t1> where $t1: RtType {
            #[inline]
            fn deref_mut(&mut self) -> &mut $pinterface {
                unsafe { ::std::mem::transmute(self) }
            }
        }
    };

    (interface $interface:ident<$t1:ident, $t2:ident> ($vtbl:ident) : $pinterface:ident ($pvtbl:ident) [$iid:ident]
        {$(
            fn $method:ident(&mut self $(,$p:ident : $t:ty)*) -> $rtr:ty
        ),+}
    ) => {
        #[repr(C)] #[allow(missing_copy_implementations)] #[doc(hidden)]
        pub struct $vtbl<$t1, $t2> where $t1: RtType, $t2: RtType {
            pub parent: $crate::$pvtbl
            $(,pub $method: unsafe extern "system" fn(
                This: *mut $interface<$t1, $t2>
                $(,$p: $t)*
            ) -> $rtr)+
        }
        #[repr(C)] #[derive(Debug)] #[allow(missing_copy_implementations)]
        pub struct $interface<$t1, $t2> where $t1: RtType, $t2: RtType {
            lpVtbl: *const $vtbl<$t1, $t2>,
        }
        impl<$t1, $t2> $interface<$t1, $t2> where $t1: RtType, $t2: RtType {
            #[inline]
            $(pub unsafe fn $method(&mut self $(,$p: $t)*) -> $rtr {
                ((*self.lpVtbl).$method)(self $(,$p)*)
            })+
        }
        impl<$t1: 'static, $t2: 'static> ComInterface for $interface<$t1, $t2> where $t1: RtType, $t2: RtType {
            type Vtbl = $vtbl<$t1, $t2>;
        }
        unsafe impl<$t1: 'static, $t2: 'static> RtInterface for $interface<$t1, $t2> where $t1: RtType, $t2: RtType {}
        impl<'a, $t1, $t2> RtType for &'a $interface<$t1, $t2> where $t1: RtType, $t2: RtType {
            type In = &'a mut $interface<$t1, $t2>;
            type Abi = *mut $interface<$t1, $t2>;
            type Out = ComPtr<$interface<$t1, $t2>>;

            unsafe fn unwrap(v: Self::In) -> Self::Abi { v }
            unsafe fn uninitialized() -> Self::Abi { ::std::ptr::null_mut() }
            unsafe fn wrap(v: Self::Abi) -> Self::Out { ComPtr::wrap(v) }
        }
        impl<$t1, $t2> ::std::ops::Deref for $interface<$t1, $t2> where $t1: RtType, $t2: RtType {
            type Target = $pinterface;
            #[inline]
            fn deref(&self) -> &$pinterface {
                unsafe { ::std::mem::transmute(self) }
            }
        }
        impl<$t1, $t2> ::std::ops::DerefMut for $interface<$t1, $t2> where $t1: RtType, $t2: RtType {
            #[inline]
            fn deref_mut(&mut self) -> &mut $pinterface {
                unsafe { ::std::mem::transmute(self) }
            }
        }
    };
}

macro_rules! RT_DELEGATE {
    // without generic parameters
    (delegate $interface:ident ($vtbl:ident, $imp:ident) [$iid:ident] {
        fn Invoke(&mut self $(,$p:ident : $t:ty)*) -> $rtr:ty
    }) => {
        RT_INTERFACE!{interface $interface($vtbl) : IUnknown(IUnknownVtbl) [$iid] {
            fn Invoke(&mut self $(,$p : $t)*) -> $rtr
        }}

        impl $interface {
			#[inline] pub fn new<_F_>(f: _F_) -> ComPtr<$interface>
				where _F_: 'static + Send + FnMut($($t),*) -> $rtr, $interface: ComIid {
				$imp::new(f).into_interface()
    		}
		}

        struct $imp<_F_> where _F_: 'static + Send + FnMut($($t),*) -> $rtr {
            invoke: _F_
        }

        impl<_F_> $imp<_F_>
            where $interface: ComIid, _F_: 'static + Send + FnMut($($t),*) -> $rtr
        {
            pub fn new(f: _F_) -> $imp<_F_> {
                $imp {
                    invoke: f,
                }
            }
        }

        impl<_F_> ::rt::handler::ComClass<$interface> for $imp<_F_>
            where $interface: ComIid, _F_: 'static + Send + FnMut($($t),*) -> $rtr
        {
            fn get_vtbl() -> $vtbl {
                $vtbl {
                    parent: ::IUnknownVtbl {
                        QueryInterface: ::rt::handler::ComReprHandler_QueryInterface::<$imp<_F_>, _>,
                        AddRef: ::rt::handler::ComRepr_AddRef::<$imp<_F_>>,
                        Release: ::rt::handler::ComRepr_Release::<$imp<_F_>>,
                    },
                    Invoke: {
                        unsafe extern "system" fn Invoke<_F_>(this_: *mut $interface $(,$p : $t)*) -> $rtr
                            where $interface: ComIid, _F_: 'static + Send + FnMut($($t),*) -> $rtr
                        {
                            let this: &mut $imp<_F_> = ::rt::handler::ComClass::from_interface(this_);
                            (this.invoke)($($p),*)
                        }
                        Invoke::<_F_>
                    }
                }
            }
        }

        /*impl<_F_> Drop for $imp<_F_>
            where _F_: 'static + Send + FnMut($($t),*) -> $rtr
        {
            fn drop(&mut self) {
                println!("DROPPED {}<...>!", stringify!($imp));
            }
        }*/
    };

    // with generic parameters
    (delegate $interface:ident<$($ht:ident),+> ($vtbl:ident, $imp:ident) [$iid:ident] {
        fn Invoke(&mut self $(,$p:ident : $t:ty)*) -> $rtr:ty
    }) => {
        RT_INTERFACE!{interface $interface<$($ht),+>($vtbl) : IUnknown(IUnknownVtbl) [$iid] {
            fn Invoke(&mut self $(,$p : $t)*) -> $rtr
        }}

        impl<$($ht: RtType + 'static),+> $interface<$($ht),+> {
			#[inline] pub fn new<_F_>(f: _F_) -> ComPtr<$interface<$($ht),+>>
				where _F_: 'static + Send + FnMut($($t),*) -> $rtr, $interface<$($ht),+>: ComIid {
				$imp::new(f).into_interface()
    		}
		}

        struct $imp<$($ht: RtType),+ , _F_> where _F_: 'static + Send + FnMut($($t),*) -> $rtr {
            invoke: _F_,
            phantom: ::std::marker::PhantomData<($($ht),+)>
        }

        impl<$($ht: RtType + 'static),+ , _F_> $imp<$($ht),+ , _F_>
            where $interface<$($ht),+>: ComIid, _F_: 'static + Send + FnMut($($t),*) -> $rtr
        {
            pub fn new(f: _F_) -> $imp<$($ht),+ , _F_> {
                $imp {
                    invoke: f,
                    phantom: ::std::marker::PhantomData
                }
            }
        }

        impl<$($ht: RtType + 'static),+ , _F_> ::rt::handler::ComClass<$interface<$($ht),+>> for $imp<$($ht),+ , _F_>
            where $interface<$($ht),+>: ComIid, _F_: 'static + Send + FnMut($($t),*) -> $rtr
        {
            fn get_vtbl() -> $vtbl<$($ht),+> {
                $vtbl::<$($ht),+> {
                    parent: ::IUnknownVtbl {
                        QueryInterface: ::rt::handler::ComReprHandler_QueryInterface::<$imp<$($ht),+ , _F_>, _>,
                        AddRef: ::rt::handler::ComRepr_AddRef::<$imp<$($ht),+ , _F_>>,
                        Release: ::rt::handler::ComRepr_Release::<$imp<$($ht),+ , _F_>>,
                    },
                    Invoke: {
                        unsafe extern "system" fn Invoke<$($ht: RtType + 'static),+ , _F_>(this_: *mut $interface<$($ht),+> $(,$p : $t)*) -> $rtr
                            where $interface<$($ht),+>: ComIid, _F_: 'static + Send + FnMut($($t),*) -> $rtr
                        {
                            let this: &mut $imp<$($ht),+ , _F_> = ::rt::handler::ComClass::from_interface(this_);
                            (this.invoke)($($p),*)
                        }
                        Invoke::<$($ht),+ , _F_>
                    }
                }
            }
        }

        /*impl<$($ht: RtType),+ , _F_> Drop for $imp<$($ht),+ , _F_>
            where _F_: 'static + Send + FnMut($($t),*) -> $rtr
        {
            fn drop(&mut self) {
                println!("DROPPED {}<...>!", stringify!($imp));
            }
        }*/
    };
}

macro_rules! RT_CLASS {
    ($cls:ident : $interface:ty) => {
        pub struct $cls($interface);
        unsafe impl RtInterface for $cls {}
        impl ComInterface for $cls {
            type Vtbl = <$interface as ComInterface>::Vtbl;
        }
        impl ComIid for $cls {
            fn iid()-> &'static ::Guid { <$interface as ComIid>::iid() }
        }
        impl<'a> RtType for &'a $cls {
            type In = &'a mut $cls;
            type Abi = *mut $cls;
            type Out = ComPtr<$cls>;
            
            unsafe fn unwrap(v: Self::In) -> Self::Abi { v }
            unsafe fn uninitialized() -> Self::Abi { ::std::ptr::null_mut() }
            unsafe fn wrap(v: Self::Abi) -> Self::Out { ComPtr::wrap(v) }
        }
    };
}


macro_rules! RT_ENUM {
    {enum $name:ident : $t:ty { $($variant:ident ($longvariant:ident) = $value:expr,)+ }} => {
        #[repr(C)] #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        #[allow(non_upper_case_globals)]
        pub struct $name(pub $t);
        $(pub const $longvariant: $name = $name($value);)+
        #[cfg(feature = "nightly")]
        impl $name {
            $(pub const $variant: $name = $name($value);)+
        }
        unsafe impl RtValueType for $name {}
    };
}

macro_rules! RT_STRUCT {
    {$(#[$attrs:meta])* struct $name:ident { $($field:ident: $ftype:ty,)* }} => {
        #[repr(C)] #[derive(Debug,Copy,Clone)] $(#[$attrs])*
        pub struct $name {
            $(pub $field: $ftype,)*
        }
        unsafe impl RtValueType for $name {}
    };
}

macro_rules! RT_PINTERFACE {
    (
        for<'a> $t:ty => [$l:expr, $w1:expr, $w2:expr, $b1:expr, $b2:expr, $b3:expr, $b4:expr, $b5:expr,
        $b6:expr, $b7:expr, $b8:expr] as $iid:ident
    ) => {
        DEFINE_IID!($iid, $l,$w1,$w2,$b1,$b2,$b3,$b4,$b5,$b6,$b7,$b8);
		impl<'a> ComIid for $t {
			fn iid()-> &'static ::Guid { &$iid }
		}
    };
    (
        for $t:ty => [$l:expr, $w1:expr, $w2:expr, $b1:expr, $b2:expr, $b3:expr, $b4:expr, $b5:expr,
        $b6:expr, $b7:expr, $b8:expr] as $iid:ident
    ) => {
        DEFINE_IID!($iid, $l,$w1,$w2,$b1,$b2,$b3,$b4,$b5,$b6,$b7,$b8);
		impl ComIid for $t {
			fn iid()-> &'static ::Guid { &$iid }
		}
    };
}

pub mod handler;
pub mod gen; // import auto-generated definitions

pub use self::gen::windows::foundation::collections::{
    IIterable,
    IIterator,
    IVectorView
};
pub use self::gen::windows::foundation::{
    IAsyncAction,
    IAsyncOperation,
    AsyncStatus,
    AsyncOperationCompletedHandler
};

// FIXME: maybe better reexport from winapi?
DEFINE_IID!(IID_IInspectable, 0xAF86E2E0, 0xB12D, 0x4c6a, 0x9C, 0x5A, 0xD7, 0xAA, 0x65, 0x10, 0x1E, 0x90);
RT_INTERFACE!{
interface IInspectable(IInspectableVtbl): IUnknown(IUnknownVtbl) [IID_IInspectable]  {
    fn GetIids(&mut self, iidCount: *mut ULONG, iids: *mut *mut IID) -> HRESULT,
    fn GetRuntimeClassName(&mut self, className: *mut HSTRING) -> HRESULT,
    fn GetTrustLevel(&mut self, trustLevel: *mut TrustLevel) -> HRESULT
}}

// =========================================== //
// Everything below will eventually be deleted //
// =========================================== //

DEFINE_IID!(IID_IMidiOutPortStatics, 106742761, 3976, 17547, 155, 100, 169, 88, 38, 198, 91, 143);
RT_INTERFACE!{interface IMidiOutPortStatics(IMidiOutPortStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IMidiOutPortStatics] {
    fn FromIdAsync(&mut self, deviceId: HSTRING, asyncOp: *mut *mut IAsyncOperation<&IMidiOutPort>) -> HRESULT,
    fn GetDeviceSelector(&mut self, value: *const HSTRING) -> HRESULT
}}

DEFINE_IID!(IID_IDeviceInformationStatics, 3246329870, 14918, 19064, 128, 19, 118, 157, 201, 185, 115, 144);
RT_INTERFACE!{interface IDeviceInformationStatics(IDeviceInformationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IDeviceInformationStatics] {
    fn __CreateFromIdAsync(&mut self) -> HRESULT,
    fn __CreateFromIdAsyncAdditionalProperties(&mut self) -> HRESULT,
    fn FindAllAsync(&mut self, asyncOp: *mut *mut IAsyncOperation<&IVectorView<&IDeviceInformation>>) -> HRESULT,
    fn __FindAllAsyncDeviceClass(&mut self) -> HRESULT,
    fn FindAllAsyncAqsFilter(&mut self, aqsFilter: HSTRING, asyncOp: *mut *mut IAsyncOperation<&IVectorView<&IDeviceInformation>>) -> HRESULT,
    fn __FindAllAsyncAqsFilterAndAdditionalProperties(&mut self) -> HRESULT,
    fn __CreateWatcher(&mut self) -> HRESULT,
    fn __CreateWatcherDeviceClass(&mut self) -> HRESULT,
    fn __CreateWatcherAqsFilter(&mut self) -> HRESULT,
    fn __CreateWatcherAqsFilterAndAdditionalProperties(&mut self) -> HRESULT
}}

// "Specialize" the IID of IIterable for a given parameter type
impl<'a> ComIid for IIterable<&'a IDeviceInformation> {
    // const IID: ::w::REFIID = &IID_IIterable_1__Windows_Devices_Enumeration_DeviceInformation;
    fn iid() -> &'static ::Guid {
        &IID_IIterable_1__Windows_Devices_Enumeration_DeviceInformation
    }
}

// This maps the logical type `DeviceInformationCollection` to its correct ABI type.
// TODO: Is a type alias sufficient? (Also see `AggregateType` in windows.foundation.collections.h)
pub type DeviceInformationCollection<'a> = IVectorView<&'a IDeviceInformation>;

DEFINE_IID!(IID_IIterable_1__Windows_Devices_Enumeration_DeviceInformation, 0xdd9f8a5d, 0xec98, 0x5f4b, 0xa3, 0xea, 0x9c, 0x8b, 0x5a, 0xd5, 0x3c, 0x4b);

// These parametrized GUIDs can be automatically generated
DEFINE_IID!(IID_AsyncOperationCompletedHandler_1_Windows_Devices_Enumeration_DeviceInformationCollection, 0x4A458732, 0x527E, 0x5C73, 0x9A, 0x68, 0xA7, 0x3D, 0xA3, 0x70, 0xF7, 0x82);

impl<'a> ComIid for AsyncOperationCompletedHandler<&'a DeviceInformationCollection<'a>> {
    // const IID: ::w::REFIID = &IID_IAsyncOperationCompletedHandler_1_Windows_Devices_Enumeration_DeviceInformationCollection;
    fn iid() -> &'static ::Guid {
        &IID_AsyncOperationCompletedHandler_1_Windows_Devices_Enumeration_DeviceInformationCollection
    }
}

DEFINE_IID!(IID_IDeviceInformation, 2879454101, 17304, 18589, 142, 68, 230, 19, 9, 39, 1, 31);
RT_INTERFACE!{interface IDeviceInformation(IDeviceInformationVtbl): IInspectable(IInspectableVtbl) [IID_IDeviceInformation] {
    fn get_Id(&mut self, value: *mut HSTRING) -> HRESULT,
    fn get_Name(&mut self, value: *mut HSTRING) -> HRESULT
// ...
}}

DEFINE_IID!(IID_IMidiOutPort, 2468179359, 22434, 19002, 173, 184, 70, 64, 136, 111, 102, 147);
RT_INTERFACE!{interface IMidiOutPort(IMidiOutPortVtbl): IInspectable(IInspectableVtbl) [IID_IDeviceInformation] {
    fn __Dummy(&mut self) -> HRESULT
// ...
}}