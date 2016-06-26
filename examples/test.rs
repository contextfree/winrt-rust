#![allow(non_snake_case)]

extern crate winapi as w;
extern crate winrt as wrt;
// TODO: don't use functions from runtimeobject directly 
extern crate runtimeobject;

use std::ptr;

use wrt::*;
use runtimeobject::*;

// TODO: re-export necessary types from winapi
use ::w::{
    HRESULT,
    VOID,
    S_OK,
    TRUE,
    FALSE,
    RO_INIT_MULTITHREADED,
};

use wrt::windows::foundation::*;
use wrt::windows::devices::enumeration::*;

fn main() {
    unsafe {
        let hres = RoInitialize(RO_INIT_MULTITHREADED);
        println!("HRESULT (RoInitialize) = {}", hres);
        let mut f: ::w::UINT32 = 0;
        assert!(RoGetErrorReportingFlags(&mut f) == S_OK);
        println!("ErrorReportingFlags: {:?}", f);
        run();
        RoUninitialize();
    }
}

fn run() {
    use std::sync::{Arc, Mutex, Condvar};
    
    let Windows_Devices_Midi_MidiOutPort: FastHString = "Windows.Devices.Midi.MidiOutPort".into();
    let mut outPortStatics = unsafe {
        let mut res = ptr::null_mut();
        let hres = RoGetActivationFactory(Windows_Devices_Midi_MidiOutPort.get_ref().get(), &IMidiOutPortStatics::iid().as_iid(), &mut res as *mut *mut _ as *mut *mut VOID);
        assert_eq!(hres, S_OK);
        ComPtr::<IMidiOutPortStatics>::wrap(res)
    };
    //println!("outPortStatics: {}", outPortStatics.get_runtime_class_name()); // this is not allowed
    
    let deviceSelector = unsafe {
        let mut res = ptr::null_mut();
        assert_eq!(outPortStatics.GetDeviceSelector(&mut res), S_OK);
        HString::wrap(res)
    };
    println!("{}", deviceSelector);
    
    let Windows_Devices_Enumeration_DeviceInformation: FastHString = "Windows.Devices.Enumeration.DeviceInformation".into();
    let mut deviceInformationStatics = unsafe {
        let mut res = ptr::null_mut();
        let hres = RoGetActivationFactory(Windows_Devices_Enumeration_DeviceInformation.get_ref().get(), &IDeviceInformationStatics::iid().as_iid(), &mut res as *mut *mut _ as *mut *mut VOID);
        println!("HRESULT (deviceInformationStatics) = {}", hres);
        ComPtr::<IDeviceInformationStatics>::wrap(res)
    };
    
    let mut asyncOp = unsafe {
        let mut res = ptr::null_mut();
        // Test some error reporting by using an invalid device selector
        let wrongDeviceSelector: FastHString = "Foobar".into();
        let hres = deviceInformationStatics.FindAllAsyncAqsFilter(wrongDeviceSelector.get_ref().get(), &mut res);
        println!("HRESULT (FindAllAsync) = {}", hres);

        let mut errorInfo = {
            let mut res = ptr::null_mut();
            assert_eq!(GetRestrictedErrorInfo(&mut res), S_OK);
            ComPtr::wrap(res)
        };
        let (description, error, restrictedDescription, _) = {
            let mut description = ptr::null_mut();
            let mut error: HRESULT = 0;
            let mut restrictedDescription = ptr::null_mut();
            let mut capabilitySid = ptr::null_mut();
            assert_eq!(errorInfo.GetErrorDetails(&mut description, &mut error, &mut restrictedDescription, &mut capabilitySid), S_OK);
            (BStr::wrap(description), error, BStr::wrap(restrictedDescription), BStr::wrap(capabilitySid))
        };
        println!("Got Error Info: {} ({})", description, restrictedDescription);
        assert_eq!(error, hres); // the returned HRESULT within IRestrictedErrorInfo is the same as the original HRESULT
        // NOTE: `res` is still null pointer at this point

        //assert_eq!(deviceInformationStatics.FindAllAsyncAqsFilter(deviceSelector.get_ref().get(), &mut res), S_OK);
        assert_eq!(deviceInformationStatics.FindAllAsync(&mut res), S_OK);
        
        ComPtr::wrap(res)
    };
    
    println!("CLS: {}",  asyncOp.get_runtime_class_name());
    
    let mut asi = asyncOp.query_interface::<IAsyncInfo>().unwrap();
    println!("IAsyncInfo: {:p}, IAsyncOperation: {:p}", asi, asyncOp);
    
    let unknown = asyncOp.query_interface::<IUnknown>().unwrap();
    println!("IAsyncInfo: {:p}, IAsyncOperation: {:p}, IUnknown: {:p}", asi, asyncOp, unknown);
    
    let unknown = asi.query_interface::<IUnknown>().unwrap();
    println!("IAsyncInfo: {:p}, IAsyncOperation: {:p}, IUnknown: {:p}", asi, asyncOp, unknown);
    
    let id = unsafe {
        let mut res = 0;
        assert!(asi.get_Id(&mut res) == S_OK);
        res
    };
    println!("id: {:?}", id);
    
    let status = unsafe {
        let mut res = std::mem::uninitialized();
        assert_eq!(asi.get_Status(&mut res), S_OK);
        res
    };
    println!("status: {:?}", status);
    
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    {
        let pair2 = pair.clone();
        let mut myHandler = AsyncOperationCompletedHandlerImpl::new(move |_op, status| {
            println!("Result handler invoked! Status: {:?}", status);
            let &(ref lock, ref cvar) = &*pair2;
            let mut started = lock.lock().unwrap();
            *started = true;
            cvar.notify_one();
            S_OK
        }).into_interface();
        assert_eq!(unsafe { asyncOp.put_Completed(&mut *myHandler) }, S_OK);
        // local reference to myHandler is dropped here -> Release() is called
    }
    
    println!("Waiting for results of async call ...");
    
    // use condvar to wait until handler has been called
    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    let mut deviceInformationCollection = unsafe {
        let mut res = ptr::null_mut();
        assert!(asyncOp.GetResults(&mut res) == S_OK);
        ComPtr::wrap(res)
    };
    println!("CLS: {}", deviceInformationCollection.get_runtime_class_name());
    let mut count = 0;
    assert_eq!(unsafe { deviceInformationCollection.get_Size(&mut count) }, S_OK);
    println!("Device Count: {}", count);
    
    let mut remember = None;
    let mut i = 0;
    for mut current in &mut deviceInformationCollection { // without the `&mut` it would consume the ComPtr
        let deviceName = unsafe {
            let mut res = ptr::null_mut();
            current.get_Name(&mut res);
            HString::wrap(res)
        };
        println!("Device Name ({}): {}", i, deviceName);
        if i == 100 {
            // remember the 100th value and try to find it later using IndexOf
            remember = Some(current);
        }
        i += 1;
    }
    assert_eq!(i, count);
    let mut index = 0;
    let mut found = FALSE;
    if let Some(mut r) = remember {
        assert_eq!(unsafe { deviceInformationCollection.IndexOf(&mut *r, &mut index, &mut found) }, S_OK);
        println!("Found remembered value: {} (index: {})", found == TRUE, index);
    }
    
    unsafe {
        let mut res = ptr::null_mut();
        let hres = deviceInformationCollection.GetAt(2000, &mut res);
        println!("HRESULT (GetAt) = {:x}", hres); // will be E_BOUNDS (out of bounds)
    }
    
    let status = unsafe {
        let mut res = std::mem::uninitialized();
        assert_eq!(asi.get_Status(&mut res), S_OK);
        res
    };
    println!("status: {:?}", status);
    
    let hres = unsafe { asi.Close() };
    println!("HRESULT (Close AsyncInfo) = {:x}", hres);
}