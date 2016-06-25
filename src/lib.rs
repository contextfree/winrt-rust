#![cfg_attr(test,feature(test))]

#![cfg_attr(feature = "nightly", feature(specialization))]
#![cfg_attr(feature = "nightly", feature(associated_consts))]

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

// TODO: get rid of manually listing re-exports (how to integrate handlers?)
pub mod windows {
    pub mod foundation {
        pub use handler::{AsyncOperationCompletedHandlerImpl};
        pub use rt::gen::windows::foundation::*;
    }
    pub mod devices {
        pub mod enumeration {
            pub use rt::{IDeviceInformationStatics, IDeviceInformation, DeviceInformationCollection, IMidiOutPortStatics, IMidiOutPort};
            // TODO: What about IIDs and statics?
            pub use rt::{IID_IMidiOutPortStatics, IID_IDeviceInformationStatics};
        }
    }
}