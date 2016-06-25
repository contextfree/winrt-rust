#![cfg_attr(test,feature(test))]

#![cfg_attr(feature = "nightly",feature(specialization))]

#![allow(dead_code,non_upper_case_globals,non_snake_case)]

#[macro_use(DEFINE_GUID)] extern crate winapi as w;
extern crate runtimeobject;
extern crate oleaut32;

mod hstring;
pub use hstring::{HString, FastHString, HStringRef};
mod bstr;
pub use bstr::BStr;

mod comptr;
pub use comptr::ComPtr;

mod cominterfaces;
pub use cominterfaces::*;

mod rt;
pub use rt::{RtInterface, RtValueType, RtType, IInspectable, IInspectableVtbl};

mod handler;
pub use handler::IntoInterface;

// TODO: use lower-case (Rust style) or upper-case (WinRT style) module names?
// TODO: auto-generate these re-exports
pub mod windows {
    pub mod foundation {
        pub use handler::{AsyncOperationCompletedHandlerImpl}; // TODO: hide this type
        pub use rt::generated::{
            Windows_Foundation_IStringable as IStringable,
            Windows_Foundation_IAsyncInfo as IAsyncInfo,
            Windows_Foundation_IAsyncAction as IAsyncAction,
            Windows_Foundation_IAsyncAction as IAsyncOperation,
            Windows_Foundation_AsyncStatus as AsyncStatus,
            Windows_Foundation_AsyncActionCompletedHandler as AsyncActionCompletedHandler,
        };
        pub mod collections {
            pub use rt::generated::{
                Windows_Foundation_Collections_IIterable as IIterable,
                Windows_Foundation_Collections_IIterator as IIterator,
                Windows_Foundation_Collections_IVectorView as IVectorView,
            };
        }
    }
    pub mod devices {
        pub mod enumeration {
            pub use rt::{IDeviceInformationStatics, IDeviceInformation, DeviceInformationCollection, IMidiOutPortStatics, IMidiOutPort};
            // TODO: What about IIDs and statics?
            pub use rt::{IID_IMidiOutPortStatics, IID_IDeviceInformationStatics};
        }
    }
}