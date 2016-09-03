#![cfg(windows)]

#![cfg_attr(test,feature(test))]

#![cfg_attr(feature = "nightly", feature(specialization))]
#![cfg_attr(feature = "nightly", feature(associated_consts))]

#![allow(dead_code,non_upper_case_globals,non_snake_case)]

extern crate winapi as w;
extern crate runtimeobject;
extern crate ole32;
extern crate oleaut32;

mod guid;
pub use guid::Guid;

///Represents the trust level of an activatable class (re-export from WinAPI crate)
pub type TrustLevel = ::w::TrustLevel;

// Compared to the DEFINE_GUID macro from winapi, this one creates a private const
macro_rules! DEFINE_IID {
    (
        $name:ident, $l:expr, $w1:expr, $w2:expr, $b1:expr, $b2:expr, $b3:expr, $b4:expr, $b5:expr,
        $b6:expr, $b7:expr, $b8:expr
    ) => {
        const $name: &'static ::Guid = &::Guid {
            Data1: $l,
            Data2: $w1,
            Data3: $w2,
            Data4: [$b1, $b2, $b3, $b4, $b5, $b6, $b7, $b8],
        };
    }
}

mod hstring;
pub use hstring::{HString, FastHString, HStringReference, HStringArg};
mod bstr;
pub use bstr::BStr;

mod comptr;
pub use comptr::{ComPtr, ComArray};

mod cominterfaces;
pub use cominterfaces::{ComInterface, ComIid, IUnknown, IRestrictedErrorInfo, IAgileObject};

mod rt;
pub use rt::{RtInterface, RtClassInterface, RtValueType, RtType, RtActivatable, IInspectable, IInspectableVtbl, Char, RuntimeContext};

mod result;
pub use result::{Result, Error, HRESULT};

pub mod windows {
    pub use rt::gen::windows::*;
}

/// This is only for internal use within the generated code
mod prelude {
    pub use ::rt::{RtType, IInspectable, IInspectableVtbl, Char};
    pub use ::rt::handler::IntoInterface;
    pub use ::cominterfaces::{ComInterface, ComIid, IUnknown};
    pub use ::comptr::{ComPtr, ComArray};
    pub use ::hstring::{HString, HStringArg};
    pub use ::result::{Result, HRESULT};
    pub use ::w::{IUnknownVtbl, S_OK, HSTRING};
    pub use ::std::ptr::null_mut;
    pub use ::std::mem::zeroed;
    pub use ::guid::Guid;

    #[inline]
    pub fn err<T>(hr: ::result::HRESULT) -> ::result::Result<T> {
        Err(::result::Error::from_hresult(hr))
    }
}