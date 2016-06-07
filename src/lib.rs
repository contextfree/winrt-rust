#![cfg_attr(test,feature(test))]

#![cfg_attr(feature = "nightly",feature(specialization))]

#![allow(dead_code,non_upper_case_globals,non_snake_case)]

#[macro_use(DEFINE_GUID)] extern crate winapi as w;
extern crate runtimeobject;
extern crate oleaut32;

mod hstring;
pub use hstring::HString;
mod bstr;
pub use bstr::BStr;

mod comptr;
pub use comptr::ComPtr;

mod cominterfaces;
pub use cominterfaces::*;

mod rt;
pub use rt::{RtInterface, RtValueType, RtType, RtGetUninitialized, IInspectable, IInspectableVtbl};

mod handler;
pub use handler::IntoInterface;

// This is a helper which also helps us to make type inference work ...
pub fn out<T>(ptr: &mut T) -> &mut <T as ComGetPtr>::Abi where T: ComGetPtr {
    ptr.get_address()
}

// TODO: use lower-case (Rust style) or upper-case (WinRT style) module names?
pub mod windows {
    pub mod foundation {
        pub use rt::{IIterable, IIterator, IVectorView, IStringable, IAsyncInfo, IAsyncAction, IAsyncActionCompletedHandler, IAsyncOperation, IAsyncOperationCompletedHandler, AsyncStatus};
        pub use handler::{AsyncOperationCompletedHandler};
    }
    pub mod devices {
        pub mod enumeration {
            pub use rt::{IDeviceInformationStatics, IMidiOutPortStatics, IMidiOutPort};
            // TODO: What about IIDs and statics?
            pub use rt::{IID_IMidiOutPortStatics, IID_IDeviceInformationStatics};
        }
    }
}