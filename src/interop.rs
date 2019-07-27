#[allow(unused_macros)]
#[macro_export]
macro_rules! COM_INTERFACE {
    // version with no methods
    ($(#[$attr:meta])* interface $interface:ident ($vtbl:ident) : $pinterface:ident [$iid:ident]
        {}
    ) => {
        #[repr(transparent)] #[allow(missing_copy_implementations)] #[doc(hidden)]
        pub struct $vtbl {
            pub parent: <<$pinterface as $crate::ComInterface>::TAbi as $crate::ComInterfaceAbi>::Vtbl
        }
        $(#[$attr])* #[repr(transparent)] #[derive(Clone)]
        pub struct $interface($crate::ComPtr<$crate::ComAbi<$vtbl>>);
        impl $crate::ComIid for $interface {
            #[inline] fn iid() -> &'static crate::Guid { &$iid }
        }
        impl $crate::ComInterface for $interface {
            type TAbi = $crate::ComAbi<$vtbl>;
            #[inline] unsafe fn wrap_com(ptr: *mut Self::TAbi) -> Self { $interface($crate::ComPtr::wrap(ptr)) }
            #[inline] fn get_abi(&self) -> &Self::TAbi { self.0.as_abi() }
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

    // version with methods
    ($(#[$attr:meta])* interface $interface:ident ($vtbl:ident) : $pinterface:ident [$iid:ident]
        {$(
            $(#[cfg($cond_attr:meta)])* fn $method:ident(&mut self $(,$p:ident : $t:ty)*) -> $rtr:ty
        ),+}
    ) => {
        #[repr(C)] #[allow(missing_copy_implementations)] #[doc(hidden)]
        pub struct $vtbl {
            pub parent: <<$pinterface as $crate::ComInterface>::TAbi as $crate::ComInterfaceAbi>::Vtbl
            $(, $(#[cfg($cond_attr)])* pub $method: unsafe extern "system" fn(
                This: *mut $interface
                $(,$p: $t)*
            ) -> $rtr)+
        }
        $(#[$attr])* #[repr(transparent)] #[derive(Clone)]
        pub struct $interface($crate::ComPtr<$crate::ComAbi<$vtbl>>);
        impl $interface {
            #[inline]
            $(pub unsafe fn $method(&mut self $(,$p: $t)*) -> $rtr {
                let abi = $crate::ComInterface::get_abi(&*self);
                ((*$crate::ComInterfaceAbi::get_vtbl(&*abi)).$method)(
                    abi as *const _ as *mut _ $(,$p)*
                )
            })+
        }
        impl $crate::ComIid for $interface {
            #[inline] fn iid() -> &'static crate::Guid { &$iid }
        }
        impl $crate::ComInterface for $interface {
            type TAbi = $crate::ComAbi<$vtbl>;
            #[inline] unsafe fn wrap_com(ptr: *mut Self::TAbi) -> Self { $interface($crate::ComPtr::wrap(ptr)) }
            #[inline] fn get_abi(&self) -> &Self::TAbi { self.0.as_abi() }
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
}