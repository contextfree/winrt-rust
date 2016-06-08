use ::w::REFIID;

pub trait ComInterface {
    type Vtbl: 'static + Sized;
}

pub trait ComIid {
    // TODO: use associated constant once that is stable
    //const IID: REFIID;
    fn get_iid() -> REFIID;
}

// extend some definitions from winapi (re-export existing types where possible!)
DEFINE_GUID!(IID_IUnknown, 0x00000000, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46);
pub type IUnknown = ::w::IUnknown;
pub type IUnknownVtbl = ::w::IUnknownVtbl;
impl ComIid for IUnknown { /*const IID: REFIID = &IID_IUnknown;*/ fn get_iid() -> REFIID { &IID_IUnknown } }
impl ComInterface for IUnknown { type Vtbl = IUnknownVtbl; }

DEFINE_GUID!(IID_IRestrictedErrorInfo, 0x82BA7092, 0x4C88, 0x427D, 0xA7, 0xBC, 0x16, 0xDD, 0x93, 0xFE, 0xB6, 0x7E);
pub type IRestrictedErrorInfo = ::w::IRestrictedErrorInfo;
pub type IRestrictedErrorInfoVtbl = ::w::IRestrictedErrorInfoVtbl;
impl ComIid for IRestrictedErrorInfo { /*const IID: REFIID = &IID_IInspectable;*/ fn get_iid() -> REFIID { &IID_IRestrictedErrorInfo } }
impl ComInterface for IRestrictedErrorInfo { type Vtbl = IRestrictedErrorInfoVtbl; }

DEFINE_GUID!(IID_IAgileObject, 0x94EA2B94, 0xE9CC, 0x49E0, 0xC0, 0xFF, 0xEE, 0x64, 0xCA, 0x8F, 0x5B, 0x90);
// TODO: Add IAgileObject interface definition