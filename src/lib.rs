//! Using Windows Runtime APIs from Rust.
//!
//! ## Example
//! ```
//! extern crate winrt;
//!
//! use winrt::*; // import various helper types
//! use winrt::windows::system::diagnostics::*; // import namespace Windows.System.Diagnostics
//!
//! fn main() {
//!     let infos = ProcessDiagnosticInfo::get_for_processes().unwrap().unwrap();
//!     println!("Currently executed processes ({}):", infos.get_size().unwrap());
//!     for p in &infos {
//!         let p = p.unwrap();
//!         let pid = p.get_process_id().unwrap();
//!         let exe = p.get_executable_file_name().unwrap();
//!         println!("[{}] {}", pid, exe);
//!     }
//! }

#![cfg(windows)]

#![cfg_attr(test,feature(test))]

#![cfg_attr(feature = "nightly", feature(specialization))]

#![allow(dead_code,non_upper_case_globals,non_snake_case)]

#[cfg(doctest)]
#[macro_use]
extern crate doc_comment;
extern crate winapi as w;

#[cfg(doctest)]
doctest!("../README.md");

mod guid;
pub use guid::Guid;

/// Represents the trust level of an activatable class (re-export from WinAPI crate)
pub type TrustLevel = w::winrt::inspectable::TrustLevel;

// Compared to the DEFINE_GUID macro from winapi, this one creates a private const
#[macro_export]
macro_rules! DEFINE_IID {
    (
        $name:ident, $l:expr, $w1:expr, $w2:expr, $b1:expr, $b2:expr, $b3:expr, $b4:expr, $b5:expr,
        $b6:expr, $b7:expr, $b8:expr
    ) => {
        const $name: &'static crate::Guid = &crate::Guid {
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
pub use comptr::{ComPtr, ComArray, ComAbi};

mod cominterfaces;
pub use cominterfaces::{ComInterface, ComInterfaceAbi, ComIid, IUnknown, IRestrictedErrorInfo, IAgileObject};

pub mod interop;

mod rt;
pub use rt::{RtInterface, RtClassInterface, RtNamedClass, RtValueType, RtType, RtActivatable,
             RtDefaultConstructible, IInspectable, IInspectableVtbl, IActivationFactory,
             IMemoryBufferByteAccess, Char, IteratorAdaptor,
             ApartmentType, init_apartment, uninit_apartment};
pub use rt::async_util::{RtAsyncAction, RtAsyncOperation};

mod result;
pub use result::{Result, Error, HRESULT};

pub mod windows {
    pub use crate::rt::gen::windows::*;
}

/// This is only for internal use within the generated code
mod prelude {
    pub use crate::rt::{RtType, RtActivatable, IInspectable, IInspectableVtbl, IActivationFactory, Char};
    pub use crate::rt::handler::IntoInterface;
    pub use crate::cominterfaces::{ComInterface, ComIid, IUnknown};
    pub use crate::comptr::ComArray;
    pub use crate::hstring::{HString, HStringArg};
    pub use crate::result::{Result, HRESULT};
    pub use w::winrt::hstring::HSTRING;
    pub use w::shared::winerror::S_OK;
    pub use w::um::unknwnbase::IUnknownVtbl;
    pub use std::ptr::null_mut;
    pub use std::mem::zeroed;
    pub use crate::guid::Guid;
    pub use crate::rt::gen::windows::foundation;
    pub(crate) use crate::comptr::ComPtr;

    #[inline]
    pub fn err<T>(hr: crate::result::HRESULT) -> crate::result::Result<T> {
        Err(crate::result::Error::from_hresult(hr))
    }
}
