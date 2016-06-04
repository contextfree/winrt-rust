#![cfg_attr(test,feature(test))]

#![feature(specialization, associated_consts)]

#![allow(dead_code,non_upper_case_globals,non_snake_case)]

#[macro_use(DEFINE_GUID)] extern crate winapi;
extern crate runtimeobject;

use ::winapi::{
    HRESULT,
    HSTRING,
    VOID,
    REFIID,
    S_OK,
    ULONG,
    BOOL,
    UINT,
    RO_INIT_MULTITHREADED
};

use ::runtimeobject::*;


pub mod hstring;
use hstring::HString;

pub mod comptr;
use comptr::ComPtr;
 
pub trait ComInterface {
    const IID: REFIID;
    type Vtbl: 'static + Sized;
}

macro_rules! RIDL {
    (interface $interface:ident ($vtbl:ident) [$iid:ident]
        {$(
            fn $method:ident(&mut self $(,$p:ident : $t:ty)*) -> $rtr:ty
        ),+}
    ) => {
        #[repr(C)] #[allow(missing_copy_implementations)]
        pub struct $vtbl {
            $(pub $method: unsafe extern "system" fn(
                This: *mut $interface
                $(,$p: $t)*
            ) -> $rtr),+
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
        impl ComInterface for $interface {
            const IID: REFIID = &$iid;
            type Vtbl = $vtbl;
        }
    };
    (interface $interface:ident ($vtbl:ident) : $pinterface:ident ($pvtbl:ident) [$iid:ident] {
    }) => {
        #[repr(C)] #[allow(missing_copy_implementations)]
        pub struct $vtbl {
            pub parent: $crate::$pvtbl
        }
        #[repr(C)] #[derive(Debug)] #[allow(missing_copy_implementations)]
        pub struct $interface {
            pub lpVtbl: *const $vtbl
        }
        impl ComInterface for $interface {
            const IID: REFIID = &$iid; 
            type Vtbl = $vtbl;
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
        impl ComInterface for $interface {
            const IID: REFIID = &$iid; 
            type Vtbl = $vtbl;
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
}

// redefine some things from winapi
pub type IUnknown = ::winapi::IUnknown;
pub type IUnknownVtbl = ::winapi::IUnknownVtbl;
pub type IInspectable = ::winapi::IInspectable;
pub type IInspectableVtbl = ::winapi::IInspectableVtbl;

DEFINE_GUID!(IID_IUnknown, 0x00000000, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46);
DEFINE_GUID!(IID_IInspectable, 0xAF86E2E0, 0xB12D, 0x4c6a, 0x9C, 0x5A, 0xD7, 0xAA, 0x65, 0x10, 0x1E, 0x90);
DEFINE_GUID!(IID_IAgileObject, 0x94EA2B94, 0xE9CC, 0x49E0, 0xC0, 0xFF, 0xEE, 0x64, 0xCA, 0x8F, 0x5B, 0x90);

impl ComInterface for IUnknown { const IID: REFIID = &IID_IUnknown; type Vtbl = IUnknownVtbl; }
impl ComInterface for IInspectable { const IID: REFIID = &IID_IInspectable; type Vtbl = IInspectableVtbl; }


fn main() {
    unsafe {
        let hres = RoInitialize(RO_INIT_MULTITHREADED);
        println!("HRESULT (RoInitialize) = {}", hres);
        run();
        RoUninitialize();
    }
}

unsafe fn run() {
    use std::sync::{Arc, Mutex, Condvar};
    
    let Windows_Devices_Midi_MidiOutPort: HString = "Windows.Devices.Midi.MidiOutPort".into();    
    let mut outPortStatics: ComPtr<IMidiOutPortStatics> = ComPtr::new(ptr::null_mut());
    let hres = RoGetActivationFactory(Windows_Devices_Midi_MidiOutPort.get(), &IID_IMidiOutPortStatics, outPortStatics.get_address() as *mut *mut _ as *mut *mut VOID);
    assert_eq!(hres, S_OK);
    println!("outPortStatics: {:p}", outPortStatics);
    
    //let mut hstring: HSTRING = ptr::null_mut();
    let mut deviceSelector = HString::empty();
    let hres = outPortStatics.GetDeviceSelector(deviceSelector.get_address());
    println!("HRESULT (GetDeviceSelector) = {}", hres);
    println!("{}", deviceSelector.to_string());
    
    let Windows_Devices_Enumeration_DeviceInformation: HString = "Windows.Devices.Enumeration.DeviceInformation".into();
    let mut deviceInformationStatics: ComPtr<IDeviceInformationStatics> = ComPtr::new(ptr::null_mut());
    let hres = RoGetActivationFactory(Windows_Devices_Enumeration_DeviceInformation.get(), &IID_IDeviceInformationStatics, deviceInformationStatics.get_address() as *mut *mut _ as *mut *mut VOID);
    println!("HRESULT (deviceInformationStatics) = {}", hres);
    let mut asyncOp: ComPtr<IAsyncOperation> = ComPtr::new(ptr::null_mut());
    let hres = deviceInformationStatics.FindAllAsyncAqsFilter(deviceSelector.get(), asyncOp.get_address());
    println!("HRESULT (FindAllAsync) = {}", hres);
    
    let mut className = HString::empty();
    let hres = asyncOp.GetRuntimeClassName(className.get_address());
    println!("HRESULT (GetRuntimeClassName) = {:x}", hres);
    println!("CLS: {}", className.to_string());
    
    let mut asi = asyncOp.query_interface::<IAsyncInfo>().unwrap();
    println!("IAsyncInfo: {:p}, IAsyncOperation: {:p}", asi, asyncOp);
    
    let unknown = asyncOp.query_interface::<IUnknown>().unwrap();
    println!("IAsyncInfo: {:p}, IAsyncOperation: {:p}, IUnknown: {:p}", asi, asyncOp, unknown);
    
    let unknown = asi.query_interface::<IUnknown>().unwrap();
    println!("IAsyncInfo: {:p}, IAsyncOperation: {:p}, IUnknown: {:p}", asi, asyncOp, unknown);
    
    let mut id = 0;
    assert!(asi.get_Id(&mut id) == S_OK);
    println!("HRESULT (get_Id) = {:x}", hres);
    println!("id: {:?}", id);
    
    let mut status: AsyncStatus = AsyncStatus::Cancelled;
    
    let hres = asi.get_Status(&mut status);
    println!("HRESULT (get_Status) = {:x}", hres);
    println!("status: {:?}", status);
    
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    {
        let pair2 = pair.clone();
        let myHandler = MyBoxHandler::new(move |_op, status| {
            println!("Result handler invoked! Status: {:?}", status);
            let &(ref lock, ref cvar) = &*pair2;
            let mut started = lock.lock().unwrap();
            *started = true;
            cvar.notify_one();
            S_OK
        }).into_interface();
        assert!(asyncOp.put_Completed(myHandler.as_mut() as *mut _ as *mut IAsyncOperationCompletedHandler) == S_OK);
        // local reference to myHandler is dropped here -> Release() is called
    }
    
    // use condvar to wait until handler has been called
    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    let mut deviceInformationCollection: ComPtr<IVectorView> = ComPtr::new(ptr::null_mut());
    assert!(asyncOp.GetResults(deviceInformationCollection.get_address() as *mut _ as *mut *mut IInspectable) == S_OK);
    let mut className = HString::empty();
    assert!(deviceInformationCollection.GetRuntimeClassName(className.get_address()) == S_OK);
    println!("CLS: {}", className.to_string());
    let mut count = 0;
    assert!(deviceInformationCollection.get_Size(&mut count) == S_OK);
    println!("Device Count: {}", count);
    
    assert!(deviceInformationCollection.query_interface::<IIterable>() == None);
    
    let mut deviceInformation: ComPtr<IDeviceInformation> = ComPtr::new(ptr::null_mut());
    let hres = deviceInformationCollection.GetAt(0, deviceInformation.get_address() as *mut _ as *mut *mut IInspectable);
    println!("HRESULT (GetAt) = {:x}", hres);
    
    let mut className = HString::empty();
    deviceInformation.GetRuntimeClassName(className.get_address());
    println!("CLS: {}", className.to_string());
    
    let mut deviceName = HString::empty();
    deviceInformation.get_Name(deviceName.get_address());
    println!("Device Name (0): {}", deviceName.to_string());
    
    let hres = asi.get_Status(&mut status);
    println!("HRESULT (get_Status) = {:x}", hres);
    println!("status: {:?}", status);
    
    let hres = asi.Close();
    println!("HRESULT (Close AsyncInfo) = {:x}", hres);
}

use std::{ptr, mem};
use std::sync::atomic::Ordering;

// Custom COM component
#[repr(C)]
pub struct ComRepr<T, Vtbl> {
    vtbl: *const Vtbl,
    refcount: std::sync::atomic::AtomicUsize,
    data: T
}

/// This is a reusable implementation of AddRef that works for any ComRepr-based type
    unsafe extern "system" fn ComRepr_AddRef<T>(this: *mut IUnknown) -> ULONG
    {
        let this = this as *mut _ as *mut ComRepr<T, IUnknownVtbl>;
        
        // Increment the reference count (count member).
        let old_size = (*this).refcount.fetch_add(1, Ordering::Relaxed);
        println!("AddRef: {} -> {}", old_size, old_size  + 1);

        // We're supposed to return the updated count.
        return (old_size + 1) as ULONG;
    }

    /// This is a reusable implementation of Com_Release that works for any ComRepr-based type
    unsafe extern "system" fn ComRepr_Release<T>(this: *mut IUnknown) -> ULONG
    {
        let this = this as *mut _ as *mut ComRepr<T, IUnknownVtbl>;
        
        let old_size = (*this).refcount.fetch_sub(1, Ordering::Release);
        println!("Release: {} -> {}", old_size, old_size - 1);
        if old_size != 1 {
            return (old_size - 1) as ULONG; // return the updated count
        }
        
        std::sync::atomic::fence(Ordering::Acquire);
        //MyHandler::destroy(this as *mut IUnknown); // Arc uses a trick to call this in an inline(never) function
        Box::from_raw(this); // creates a Box which is then dropped
        return 0;
    }

pub trait ComComponent<Interface: ComInterface> where Self: Sized {
    fn get_vtbl() -> &'static Interface::Vtbl;
    fn vtbl(&self) -> &'static Interface::Vtbl {
        Self::get_vtbl()
    }
    fn into_interface(self) -> ComPtr<Interface> {
        let com = Box::new(ComRepr {
            vtbl: Self::get_vtbl(),
            refcount: std::sync::atomic::AtomicUsize::new(1),
            data: self
        });
        unsafe { ComPtr::new(Box::into_raw(com) as *mut Interface) }
    }
    unsafe fn from_interface<'a>(thing: *mut Interface) -> &'a mut Self {
        &mut (*(thing as *mut _ as *mut ComRepr<Self, Interface::Vtbl>)).data
    }
    unsafe fn from_unknown<'a>(thing: *mut IUnknown) -> &'a mut Self {
        &mut (*(thing as *mut _ as *mut ComRepr<Self, Interface::Vtbl>)).data
    }
    unsafe fn destroy(thing: *mut IUnknown) {
        Box::from_raw(thing as *mut ComRepr<Self, Interface::Vtbl>);
    }
}

struct MyHandler {
    invoke: fn(*mut IAsyncOperation, AsyncStatus) -> HRESULT
}

const MyHandlerVtbl: &'static IAsyncOperationCompletedHandlerVtbl = &IAsyncOperationCompletedHandlerVtbl {
    parent: IUnknownVtbl {
        QueryInterface: My_QueryInterface,
        AddRef: ComRepr_AddRef::<MyHandler>,
        Release: ComRepr_Release::<MyHandler>,
    },
    Invoke: {
        unsafe extern "system" fn My_Invoke(this_: *mut IAsyncOperationCompletedHandler, asyncOperation: *mut IAsyncOperation, status: AsyncStatus) -> HRESULT {
            let this: &mut MyHandler = MyHandler::from_interface(this_);
            (this.invoke)(asyncOperation, status)
        }
        My_Invoke
    }
};

impl ComComponent<IAsyncOperationCompletedHandler> for MyHandler {
    fn get_vtbl() -> &'static IAsyncOperationCompletedHandlerVtbl { &MyHandlerVtbl }
}

impl Drop for MyHandler {
    fn drop(&mut self) {
        println!("Dropped MyHandler!");
    }
}

struct MyBoxHandler {
    invoke: Box<FnMut(*mut IAsyncOperation, AsyncStatus) -> HRESULT>
}

impl MyBoxHandler {
    pub fn new<F>(f: F) -> MyBoxHandler where F: 'static + Send + FnMut(*mut IAsyncOperation, AsyncStatus) -> HRESULT {
        MyBoxHandler {
            invoke: Box::new(f)
        }
    }
}

const MyBoxHandlerVtbl: &'static IAsyncOperationCompletedHandlerVtbl = &IAsyncOperationCompletedHandlerVtbl {
    parent: IUnknownVtbl {
        QueryInterface: My_QueryInterface,
        AddRef: ComRepr_AddRef::<MyBoxHandler>,
        Release: ComRepr_Release::<MyBoxHandler>,
    },
    Invoke: {
        unsafe extern "system" fn My_Invoke(this_: *mut IAsyncOperationCompletedHandler, asyncOperation: *mut IAsyncOperation, status: AsyncStatus) -> HRESULT {
            let this: &mut MyBoxHandler = MyBoxHandler::from_interface(this_);
            (this.invoke)(asyncOperation, status)
        }
        My_Invoke
    }
};

impl ComComponent<IAsyncOperationCompletedHandler> for MyBoxHandler {
    fn get_vtbl() -> &'static IAsyncOperationCompletedHandlerVtbl { &MyBoxHandlerVtbl }
}

impl Drop for MyBoxHandler {
    fn drop(&mut self) {
        println!("Dropped MyBoxHandler!");
    }
}

unsafe extern "system" fn My_QueryInterface(this_: *mut IUnknown, vTableGuid: REFIID, ppv: *mut *mut VOID) -> HRESULT
{
    fn guid_eq(guid1: &::winapi::GUID, guid2: &::winapi::GUID) -> bool {
        guid1.Data1 == guid2.Data1 && guid1.Data2 == guid2.Data2 && guid1.Data3 == guid2.Data3 && guid1.Data4 == guid2.Data4
    }
    
    fn print_guid(guid: &::winapi::GUID) {
        println!("{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            guid.Data1, guid.Data2, guid.Data3,
            guid.Data4[0], guid.Data4[1], guid.Data4[2], guid.Data4[3],
            guid.Data4[4], guid.Data4[5], guid.Data4[6], guid.Data4[7]);
    }

    print!("QueryInterface called with GUID ");
    print_guid(&*vTableGuid);
    
    // TODO: How to determine which IIDs are allowed here?
    if !guid_eq(&*vTableGuid, &IID_IUnknown) &&
        !guid_eq(&*vTableGuid, &IID_IAgileObject) && // IAgileObject is only supported for Send objects
        !guid_eq(&*vTableGuid, &IID_IAsyncOperationCompletedHandler_1_Windows_Devices_Enumeration_DeviceInformationCollection) { 
        // We don't recognize the GUID passed to us. Let the caller know this,
        // by clearing his handle, and returning E_NOINTERFACE.
        *ppv = ptr::null_mut();
        return ::winapi::E_NOINTERFACE;
    }

    // It's a match!

    // First, we fill in his handle with the same object pointer he passed us.
    *ppv = this_ as *mut _ as *mut VOID;

    // Now we call our own AddRef function, which is easier if we first create a ComPtr wrapper
    let mut this_: ComPtr<IUnknown> = ComPtr::new(this_);
    this_.AddRef();
    mem::forget(this_); // prevent dropping the ComPtr, or otherwise we would call Release() automatically

    // Let the caller know that he indeed has an object of the requested interface.
    return S_OK;
}

DEFINE_GUID!(IID_IStringable, 2520162132, 36534, 18672, 171, 206, 193, 178, 17, 230, 39, 195);
RIDL!{interface IStringable(IStringableVtbl): IInspectable(IInspectableVtbl) [IID_IStringable] {
    fn ToString(&mut self, value: *mut HSTRING) -> HRESULT
}}

DEFINE_GUID!(IID_IAsyncInfo, 0x00000036,0x0000,0x0000,0xC0,0x00,0x00,0x00,0x00,0x00,0x00,0x46);
RIDL!{interface IAsyncInfo(IAsyncInfoVtbl): IInspectable(IInspectableVtbl) [IID_IAsyncInfo] {
    fn get_Id(&mut self, id: *mut u32) -> HRESULT,
    fn get_Status(&mut self, status: *mut AsyncStatus) -> HRESULT,
    fn get_ErrorCode(&mut self, errorCode: *mut HRESULT) -> HRESULT,
    fn Cancel(&mut self) -> HRESULT,
    fn Close(&mut self) -> HRESULT
}}

DEFINE_GUID!(IID_IAsyncAction, 0x5A648006,0x843A,0x4DA9,0x86,0x5B,0x9D,0x26,0xE5,0xDF,0xAD,0x7B);
RIDL!{interface IAsyncAction(IAsyncActionVtbl): IInspectable(IInspectableVtbl) [IID_IAsyncAction] {
    fn put_Completed(&mut self, handler: *mut IAsyncActionCompletedHandler) -> HRESULT,
    fn get_Completed(&mut self, handler: *mut *mut IAsyncActionCompletedHandler) -> HRESULT,
    fn GetResults(&mut self) -> HRESULT
}}

// see winrt/windows.foundation.h
DEFINE_GUID!(IID_IAsyncActionCompletedHandler, 0xA4ED5C81,0x76C9,0x40BD,0x8B,0xE6,0xB1,0xD9,0x0F,0xB2,0x0A,0xE7);
RIDL!{interface IAsyncActionCompletedHandler(IAsyncActionCompletedHandlerVtbl): IUnknown(IUnknownVtbl) [IID_IAsyncActionCompletedHandler] {
    fn Invoke(&mut self, asyncAction: *mut IAsyncAction, status: AsyncStatus) -> HRESULT
}}

// These parametrized GUIDs can be automatically generated
DEFINE_GUID!(IID_IAsyncOperationCompletedHandler_1_Windows_Devices_Enumeration_DeviceInformationCollection, 0x4A458732, 0x527E, 0x5C73, 0x9A, 0x68, 0xA7, 0x3D, 0xA3, 0x70, 0xF7, 0x82);
//DEFINE_GUID!(IID_IAsyncOperationCompletedHandler, ...);
RIDL!{interface IAsyncOperationCompletedHandler(IAsyncOperationCompletedHandlerVtbl): IUnknown(IUnknownVtbl) [IID_IAsyncActionCompletedHandler] {
    fn Invoke(&mut self, asyncOperation: *mut IAsyncOperation, status: AsyncStatus) -> HRESULT
}}

// TODO: this should be generic in the result type
DEFINE_GUID!(IID_IAsyncOperation, 0x9fc2b0bb, 0xe446, 0x44e2, 0xaa,0x61,0x9c,0xab,0x8f,0x63,0x6a,0xf2);
RIDL!{interface IAsyncOperation(IAsyncOperationVtbl): IInspectable(IInspectableVtbl) [IID_IAsyncOperation] {
    fn put_Completed(&mut self, handler: *mut IAsyncOperationCompletedHandler/*<TResult>*/) -> HRESULT,
    fn get_Completed(&mut self, handler: *mut *mut IAsyncOperationCompletedHandler/*<TResult>*/) -> HRESULT,
    fn GetResults(&mut self, results: *mut *mut /*TResult*/IInspectable) -> HRESULT
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
RIDL!{interface IMidiOutPortStatics(IMidiOutPortStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IMidiOutPortStatics] {
    fn FromIdAsync(&mut self, deviceId: HSTRING, asyncOp: *mut *mut IAsyncOperation/*<IMidiOutPort>*/) -> HRESULT,
    fn GetDeviceSelector(&mut self, value: *const HSTRING) -> HRESULT
}}

DEFINE_GUID!(IID_IDeviceInformationStatics, 3246329870, 14918, 19064, 128, 19, 118, 157, 201, 185, 115, 144);
RIDL!{interface IDeviceInformationStatics(IDeviceInformationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IDeviceInformationStatics] {
    fn __CreateFromIdAsync(&mut self) -> HRESULT,
    fn __CreateFromIdAsyncAdditionalProperties(&mut self) -> HRESULT,
    fn __FindAllAsync(&mut self) -> HRESULT,
    fn __FindAllAsyncDeviceClass(&mut self) -> HRESULT,
    fn FindAllAsyncAqsFilter(&mut self, aqsFilter: HSTRING, asyncOp: *mut *mut IAsyncOperation/*<DeviceInformationCollection>*/) -> HRESULT,
    fn __FindAllAsyncAqsFilterAndAdditionalProperties(&mut self) -> HRESULT,
    fn __CreateWatcher(&mut self) -> HRESULT,
    fn __CreateWatcherDeviceClass(&mut self) -> HRESULT,
    fn __CreateWatcherAqsFilter(&mut self) -> HRESULT,
    fn __CreateWatcherAqsFilterAndAdditionalProperties(&mut self) -> HRESULT
}}

DEFINE_GUID!(IID_IIterable, 4205151722, 25108, 16919, 175, 218, 127, 70, 222, 88, 105, 179);
RIDL!{interface IIterable(IIterableVtbl): IInspectable(IInspectableVtbl) [IID_IIterable] {
    fn First(&mut self, first: *mut *mut IIterator/*<T>*/) -> HRESULT
}}

DEFINE_GUID!(IID_IIterator, 1786374243, 17152, 17818, 153, 102, 203, 182, 96, 150, 62, 225);
RIDL!{interface IIterator(IIteratorVtbl): IInspectable(IInspectableVtbl) [IID_IIterator] {
    fn get_Current(&mut self, current: *mut VOID /*T*/) -> HRESULT,
    fn get_HasCurrent(&mut self, hasCurrent: *mut BOOL) -> HRESULT,
    fn MoveNext(&mut self, hasCurrent: *mut BOOL) -> HRESULT
    // fn GetMany...
}}

DEFINE_GUID!(IID_IVectorView, 3152149068, 45283, 17795, 186, 239, 31, 27, 46, 72, 62, 86);
// NOTE: For some reason this does NOT actually inherit from IIterable as the metadata would suggest
RIDL!{interface IVectorView(IVectorViewVtbl): IInspectable(IInspectableVtbl) [IID_IVectorView] {
    fn GetAt(&mut self, index: UINT, item: *mut *mut IInspectable /*T*/) -> HRESULT,
    fn get_Size(&mut self, size: *mut UINT) -> HRESULT,
    fn IndexOf(&mut self, value: VOID /*T*/, index: *mut UINT, found: *mut BOOL) -> HRESULT
    // fn GetMany...
}}

DEFINE_GUID!(IID_IDeviceInformation, 2879454101, 17304, 18589, 142, 68, 230, 19, 9, 39, 1, 31);
RIDL!{interface IDeviceInformation(IDeviceInformationVtbl): IInspectable(IInspectableVtbl) [IID_IDeviceInformation] {
    fn get_Id(&mut self, value: *mut HSTRING) -> HRESULT,
    fn get_Name(&mut self, value: *mut HSTRING) -> HRESULT
    // ...
}}