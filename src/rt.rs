use std::ptr;

use super::{ComInterface, HString, HStringRef, ComPtr, ComIid};

use ::w::{
    HRESULT,
    HSTRING,
    REFIID,
    S_OK,
    ULONG,
    BOOL,
    TRUE,
    FALSE,
    UINT,
    TrustLevel,
    IID
};

/// This means that the interfaced is based on IInspectable
pub unsafe trait RtInterface: ComInterface {}

pub unsafe trait RtValueType {}
unsafe impl RtValueType for AsyncStatus {}
// TODO ...

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
    
    unsafe fn unwrap(v: Self::In) -> Self::Abi { v.get() }
    unsafe fn uninitialized() -> Self::Abi { ::std::ptr::null_mut() }
    unsafe fn wrap(v: Self::Abi) -> Self::Out { HString::wrap(v) }
}

impl<'a> RtType for bool {
    type In = bool;
    type Abi = ::w::BOOL;
    type Out = bool;
    
    unsafe fn unwrap(v: Self::In) -> Self::Abi { if v { ::w::TRUE } else { ::w::FALSE } }
    unsafe fn uninitialized() -> Self::Abi { ::w::FALSE }
    unsafe fn wrap(v: Self::Abi) -> Self::Out { v == ::w::TRUE }
}

impl<'a> RtType for i32 {
    type In = i32;
    type Abi = ::w::INT;
    type Out = i32;
    
    unsafe fn unwrap(v: Self::In) -> Self::Abi { v }
    unsafe fn uninitialized() -> Self::Abi { 0 }
    unsafe fn wrap(v: Self::Abi) -> Self::Out { v }
}

impl<'a> RtType for u32 {
    type In = u32;
    type Abi = ::w::UINT;
    type Out = u32;
    
    unsafe fn unwrap(v: Self::In) -> Self::Abi { v }
    unsafe fn uninitialized() -> Self::Abi { 0 }
    unsafe fn wrap(v: Self::Abi) -> Self::Out { v }
}

impl<T> RtType for T where T: RtValueType {
    type In = T;
    type Abi = T;
    type Out = T;
    
    unsafe fn unwrap(v: Self::In) -> Self::Abi { v }
    unsafe fn uninitialized() -> Self::Abi { ::std::mem::zeroed() }
    unsafe fn wrap(v: Self::Abi) -> Self::Out { v }
}

// We can also implement IntoIterator for IIterable<T> and IVectorView<T> and references to those
// TODO: This should be extended to more types (at least IVector, IMap, IMapView, IObservableVector, IObservableMap)
impl<T> IntoIterator for ComPtr<IIterable<T>> where T: RtType {
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

impl<'a, T> IntoIterator for &'a mut ComPtr<IIterable<T>> where T: RtType {
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

impl<T> IntoIterator for ComPtr<IVectorView<T>> where T: RtType, IIterable<T>: ComIid {
    type Item = <T as RtType>::Out;
    type IntoIter = ComPtr<IIterator<T>>;
    fn into_iter(self) -> Self::IntoIter {
        self.query_interface::<IIterable<T>>().unwrap().into_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut ComPtr<IVectorView<T>> where T: RtType, IIterable<T>: ComIid {
    type Item = <T as RtType>::Out;
    type IntoIter = ComPtr<IIterator<T>>;
    fn into_iter(self) -> Self::IntoIter {
        self.query_interface::<IIterable<T>>().unwrap().into_iter()
    }
}

// TODO: also implement IndexMove for IVectorView etc once that exists (Index or IndexMut won't work since we can't return a reference)

impl<T> Iterator for ComPtr<IIterator<T>> where T: RtType {
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
    // Specialized version for IUnknown (does not impl RtInterface)
    (interface $interface:ident ($vtbl:ident) : IUnknown(IUnknownVtbl) [$iid:ident]
        {$(
            fn $method:ident(&mut self $(,$p:ident : $t:ty)*) -> $rtr:ty
        ),+}
    ) => {
        #[repr(C)] #[allow(missing_copy_implementations)]
        pub struct $vtbl {
            pub parent: $crate::IUnknownVtbl
            $(,pub $method: unsafe extern "system" fn(
                This: *mut $interface
                $(,$p: $t)*
            ) -> $rtr)+
        }
        #[repr(C)] #[derive(Debug)] #[allow(missing_copy_implementations)]
        pub struct $interface {
            pub lpVtbl: *const $vtbl
        }
        impl $interface {
            #[inline]
            $(pub unsafe fn $method(&mut self $(,$p: $t)*) -> $rtr {
                ((*self.lpVtbl).$method)(self $(,$p)*)
            })+
        }
        impl ComIid for $interface {
            //const IID: REFIID = &$iid;
            fn get_iid() -> REFIID { &$iid }
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
        #[repr(C)] #[allow(missing_copy_implementations)]
        pub struct $vtbl<$t1> where $t1: RtType {
            pub parent: $crate::IUnknownVtbl
            $(,pub $method: unsafe extern "system" fn(
                This: *mut $interface<$t1>
                $(,$p: $t)*
            ) -> $rtr)+
        }
        #[repr(C)] #[derive(Debug)] #[allow(missing_copy_implementations)]
        pub struct $interface<$t1> where $t1: RtType {
            pub lpVtbl: *const $vtbl<$t1>,
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
        {$(
            fn $method:ident(&mut self $(,$p:ident : $t:ty)*) -> $rtr:ty
        ),+}
    ) => {
        #[repr(C)] #[allow(missing_copy_implementations)]
        pub struct $vtbl {
            pub parent: $crate::$pvtbl
            $(,pub $method: unsafe extern "system" fn(
                This: *mut $interface
                $(,$p: $t)*
            ) -> $rtr)+
        }
        #[repr(C)] #[derive(Debug)] #[allow(missing_copy_implementations)]
        pub struct $interface {
            pub lpVtbl: *const $vtbl
        }
        impl $interface {
            #[inline]
            $(pub unsafe fn $method(&mut self $(,$p: $t)*) -> $rtr {
                ((*self.lpVtbl).$method)(self $(,$p)*)
            })+
        }
        impl ComIid for $interface {
            //const IID: REFIID = &$iid;
            fn get_iid() -> ::w::REFIID { &$iid }
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
        #[repr(C)] #[allow(missing_copy_implementations)]
        pub struct $vtbl<$t1> where $t1: RtType {
            pub parent: $crate::$pvtbl
            $(,pub $method: unsafe extern "system" fn(
                This: *mut $interface<$t1>
                $(,$p: $t)*
            ) -> $rtr)+
        }
        #[repr(C)] #[derive(Debug)] #[allow(missing_copy_implementations)]
        pub struct $interface<$t1> where $t1: RtType {
            pub lpVtbl: *const $vtbl<$t1>,
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
}

// ================================================ //
// Everything below will be automatically generated //
// ================================================ //

// FIXME: maybe better reexport from winapi?
DEFINE_GUID!(IID_IInspectable, 0xAF86E2E0, 0xB12D, 0x4c6a, 0x9C, 0x5A, 0xD7, 0xAA, 0x65, 0x10, 0x1E, 0x90);
RT_INTERFACE!{
interface IInspectable(IInspectableVtbl): IUnknown(IUnknownVtbl) [IID_IInspectable]  {
    fn GetIids(&mut self, iidCount: *mut ULONG, iids: *mut *mut IID) -> HRESULT,
    fn GetRuntimeClassName(&mut self, className: *mut HSTRING) -> HRESULT,
    fn GetTrustLevel(&mut self, trustLevel: *mut TrustLevel) -> HRESULT
}}

DEFINE_GUID!(IID_IStringable, 2520162132, 36534, 18672, 171, 206, 193, 178, 17, 230, 39, 195);
RT_INTERFACE!{interface IStringable(IStringableVtbl): IInspectable(IInspectableVtbl) [IID_IStringable] {
    fn ToString(&mut self, value: *mut ::w::HSTRING) -> ::w::HRESULT
}}

DEFINE_GUID!(IID_IAsyncInfo, 0x00000036,0x0000,0x0000,0xC0,0x00,0x00,0x00,0x00,0x00,0x00,0x46);
RT_INTERFACE!{interface IAsyncInfo(IAsyncInfoVtbl): IInspectable(IInspectableVtbl) [IID_IAsyncInfo] {
    fn get_Id(&mut self, id: *mut u32) -> ::w::HRESULT,
    fn get_Status(&mut self, status: *mut AsyncStatus) -> ::w::HRESULT,
    fn get_ErrorCode(&mut self, errorCode: *mut ::w::HRESULT) -> ::w::HRESULT,
    fn Cancel(&mut self) -> ::w::HRESULT,
    fn Close(&mut self) -> ::w::HRESULT
}}

DEFINE_GUID!(IID_IAsyncAction, 0x5A648006,0x843A,0x4DA9,0x86,0x5B,0x9D,0x26,0xE5,0xDF,0xAD,0x7B);
RT_INTERFACE!{interface IAsyncAction(IAsyncActionVtbl): IInspectable(IInspectableVtbl) [IID_IAsyncAction] {
    fn put_Completed(&mut self, handler: *mut IAsyncActionCompletedHandler) -> HRESULT,
    fn get_Completed(&mut self, handler: *mut *mut IAsyncActionCompletedHandler) -> HRESULT,
    fn GetResults(&mut self) -> HRESULT
}}

// see winrt/windows.foundation.h
DEFINE_GUID!(IID_IAsyncActionCompletedHandler, 0xA4ED5C81,0x76C9,0x40BD,0x8B,0xE6,0xB1,0xD9,0x0F,0xB2,0x0A,0xE7);
RT_INTERFACE!{interface IAsyncActionCompletedHandler(IAsyncActionCompletedHandlerVtbl): IUnknown(IUnknownVtbl) [IID_IAsyncActionCompletedHandler] {
    fn Invoke(&mut self, asyncAction: *mut IAsyncAction, status: AsyncStatus) -> HRESULT
}}

DEFINE_GUID!(IID_IAsyncOperation, 0x9fc2b0bb, 0xe446, 0x44e2, 0xaa,0x61,0x9c,0xab,0x8f,0x63,0x6a,0xf2);
RT_INTERFACE!{interface IAsyncOperation<TResult>(IAsyncOperationVtbl): IInspectable(IInspectableVtbl) [IID_IAsyncOperation] {
    fn put_Completed(&mut self, handler: *mut IAsyncOperationCompletedHandler<TResult>) -> HRESULT,
    fn get_Completed(&mut self, handler: *mut *mut IAsyncOperationCompletedHandler<TResult>) -> HRESULT,
    fn GetResults(&mut self, results: *mut TResult::Abi) -> HRESULT
}}

// These parametrized GUIDs can be automatically generated
DEFINE_GUID!(IID_IAsyncOperationCompletedHandler_1_Windows_Devices_Enumeration_DeviceInformationCollection, 0x4A458732, 0x527E, 0x5C73, 0x9A, 0x68, 0xA7, 0x3D, 0xA3, 0x70, 0xF7, 0x82);
DEFINE_GUID!(IID_IAsyncOperationCompletedHandler, 4242337836, 58840, 17528, 145, 90, 77, 144, 183, 75, 131, 165);
RT_INTERFACE!{interface IAsyncOperationCompletedHandler<TResult>(IAsyncOperationCompletedHandlerVtbl): IUnknown(IUnknownVtbl) [IID_IAsyncOperationCompletedHandler] {
    fn Invoke(&mut self, asyncOperation: *mut IAsyncOperation<TResult>, status: AsyncStatus) -> HRESULT
}}

#[repr(C)]
#[derive(Debug,PartialEq,Eq)]
pub enum AsyncStatus {
    //Created    = -1,
    Started    = 0, 
    Completed  = 1, 
    Cancelled  = 2, 
    Error      = 3 
}

DEFINE_GUID!(IID_IMidiOutPortStatics, 106742761, 3976, 17547, 155, 100, 169, 88, 38, 198, 91, 143);
RT_INTERFACE!{interface IMidiOutPortStatics(IMidiOutPortStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IMidiOutPortStatics] {
    fn FromIdAsync(&mut self, deviceId: HSTRING, asyncOp: *mut *mut IAsyncOperation<&IMidiOutPort>) -> HRESULT,
    fn GetDeviceSelector(&mut self, value: *const HSTRING) -> HRESULT
}}

DEFINE_GUID!(IID_IDeviceInformationStatics, 3246329870, 14918, 19064, 128, 19, 118, 157, 201, 185, 115, 144);
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

DEFINE_GUID!(IID_IIterable, 4205151722, 25108, 16919, 175, 218, 127, 70, 222, 88, 105, 179);
RT_INTERFACE!{interface IIterable<T>(IIterableVtbl): IInspectable(IInspectableVtbl) [IID_IIterable] {
    fn First(&mut self, first: *mut *mut IIterator<T>) -> HRESULT
}}

// "Specialize" the IID of IIterable for a given parameter type
impl<'a> ComIid for IIterable<&'a IDeviceInformation> {
    //const IID: REFIID = &IID_IIterable_1__Windows_Devices_Enumeration_DeviceInformation;
    fn get_iid() -> REFIID { &IID_IIterable_1__Windows_Devices_Enumeration_DeviceInformation }
}

// This maps the logical type `DeviceInformationCollection` to its correct ABI type.
// TODO: Is a type alias sufficient? (Also see `AggregateType` in windows.foundation.collections.h)
pub type DeviceInformationCollection<'a> = IVectorView<&'a IDeviceInformation>;

// TODO: This GUID const should be private and only accessible via the ComIid impl below
DEFINE_GUID!(IID_IIterable_1__Windows_Devices_Enumeration_DeviceInformation, 0xdd9f8a5d, 0xec98, 0x5f4b, 0xa3, 0xea, 0x9c, 0x8b, 0x5a, 0xd5, 0x3c, 0x4b);

impl<'a> ComIid for IAsyncOperationCompletedHandler<&'a DeviceInformationCollection<'a>> {
    //const IID: REFIID = &IID_IAsyncOperationCompletedHandler_1_Windows_Devices_Enumeration_DeviceInformationCollection;
    fn get_iid() -> REFIID { &IID_IAsyncOperationCompletedHandler_1_Windows_Devices_Enumeration_DeviceInformationCollection }
}

DEFINE_GUID!(IID_IIterator, 1786374243, 17152, 17818, 153, 102, 203, 182, 96, 150, 62, 225);
RT_INTERFACE!{interface IIterator<T>(IIteratorVtbl): IInspectable(IInspectableVtbl) [IID_IIterator] {
    fn get_Current(&mut self, current: *mut T::Abi) -> HRESULT,
    fn get_HasCurrent(&mut self, hasCurrent: *mut BOOL) -> HRESULT,
    fn MoveNext(&mut self, hasCurrent: *mut BOOL) -> HRESULT
    // fn GetMany...
}}

DEFINE_GUID!(IID_IVectorView, 3152149068, 45283, 17795, 186, 239, 31, 27, 46, 72, 62, 86);
// NOTE: For some reason this does NOT actually inherit from IIterable as the metadata would suggest
RT_INTERFACE!{interface IVectorView<T>(IVectorViewVtbl): IInspectable(IInspectableVtbl) [IID_IVectorView] {
    fn GetAt(&mut self, index: UINT, item: *mut T::Abi) -> HRESULT,
    fn get_Size(&mut self, size: *mut UINT) -> HRESULT,
    fn IndexOf(&mut self, value: T::Abi, index: *mut UINT, found: *mut BOOL) -> HRESULT
    // fn GetMany...
}}

DEFINE_GUID!(IID_IDeviceInformation, 2879454101, 17304, 18589, 142, 68, 230, 19, 9, 39, 1, 31);
RT_INTERFACE!{interface IDeviceInformation(IDeviceInformationVtbl): IInspectable(IInspectableVtbl) [IID_IDeviceInformation] {
    fn get_Id(&mut self, value: *mut HSTRING) -> HRESULT,
    fn get_Name(&mut self, value: *mut HSTRING) -> HRESULT
    // ...
}}

DEFINE_GUID!(IID_IMidiOutPort, 2468179359, 22434, 19002, 173, 184, 70, 64, 136, 111, 102, 147);
RT_INTERFACE!{interface IMidiOutPort(IMidiOutPortVtbl): IInspectable(IInspectableVtbl) [IID_IDeviceInformation] {
    fn __Dummy(&mut self) -> HRESULT
    // ...
}}