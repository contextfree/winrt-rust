#![cfg_attr(test,feature(test))]

#![cfg_attr(feature = "nightly", feature(specialization))]
#![cfg_attr(feature = "nightly", feature(associated_consts))]

#![allow(dead_code,non_upper_case_globals,non_snake_case)]

#[macro_use(RIDL)] extern crate winapi as w;
extern crate runtimeobject;
extern crate oleaut32;

mod guid;
pub use guid::Guid;

// Compared to the DEFINE_GUID macro from winapi, this one creates a private const
macro_rules! DEFINE_IID {
    (
        $name:ident, $l:expr, $w1:expr, $w2:expr, $b1:expr, $b2:expr, $b3:expr, $b4:expr, $b5:expr,
        $b6:expr, $b7:expr, $b8:expr
    ) => {
        const $name: &'static ::Guid = &::Guid(::w::GUID {
            Data1: $l,
            Data2: $w1,
            Data3: $w2,
            Data4: [$b1, $b2, $b3, $b4, $b5, $b6, $b7, $b8],
        });
    }
}

mod hstring;
pub use hstring::{HString, FastHString, HStringRef};
mod bstr;
pub use bstr::BStr;

mod comptr;
pub use comptr::ComPtr;

mod cominterfaces;
pub use cominterfaces::{ComInterface, ComIid, IUnknown, IUnknownVtbl, IRestrictedErrorInfo, IAgileObject};
// TODO: get rid of IUnknownVtbl export?

mod rt;
pub use rt::{RtInterface, RtValueType, RtType, RtActivatable, IInspectable, IInspectableVtbl};

pub use rt::handler::IntoInterface;

pub mod windows {
    pub use rt::gen::windows::*;
}