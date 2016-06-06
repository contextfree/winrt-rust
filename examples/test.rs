#![allow(non_snake_case)]

extern crate winapi as w;
extern crate winrt as windows;
// TODO: don't use functions from runtimeobject directly 
extern crate runtimeobject;

use windows::{out, ComPtr, BStr, HString, IRestrictedErrorInfo, IUnknown};
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

use windows::foundation::*;
use windows::devices::enumeration::*;

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

unsafe fn run() {
    use std::sync::{Arc, Mutex, Condvar};
    
    let Windows_Devices_Midi_MidiOutPort: HString = "Windows.Devices.Midi.MidiOutPort".into();
    let mut outPortStatics = ComPtr::<IMidiOutPortStatics>::uninitialized();
    let hres = RoGetActivationFactory(Windows_Devices_Midi_MidiOutPort.get(), &IID_IMidiOutPortStatics, out(&mut outPortStatics) as *mut *mut _ as *mut *mut VOID);
    assert_eq!(hres, S_OK);
    println!("outPortStatics: {:p}", outPortStatics);
    
    let mut deviceSelector = HString::empty();
    let hres = outPortStatics.GetDeviceSelector(out(&mut deviceSelector));
    println!("HRESULT (GetDeviceSelector) = {}", hres);
    println!("{}", deviceSelector.to_string());
    
    let Windows_Devices_Enumeration_DeviceInformation: HString = "Windows.Devices.Enumeration.DeviceInformation".into();
    let mut deviceInformationStatics = ComPtr::<IDeviceInformationStatics>::uninitialized();
    let hres = RoGetActivationFactory(Windows_Devices_Enumeration_DeviceInformation.get(), &IID_IDeviceInformationStatics, out(&mut deviceInformationStatics) as *mut *mut _ as *mut *mut VOID);
    println!("HRESULT (deviceInformationStatics) = {}", hres);
    
    let mut asyncOp = ComPtr::uninitialized();
    // Test some error reporting by using an invalid device selector
    let wrongDeviceSelector: HString = "Foobar".into();
    let hres = deviceInformationStatics.FindAllAsyncAqsFilter(wrongDeviceSelector.get(), out(&mut asyncOp));
    println!("HRESULT (FindAllAsync) = {}", hres);
    {
        let mut errorInfo = ComPtr::<IRestrictedErrorInfo>::uninitialized();
        assert_eq!(GetRestrictedErrorInfo(out(&mut errorInfo)), S_OK);
        let mut description = BStr::empty();
        let mut error: HRESULT = 0;
        let mut restrictedDescription = BStr::empty();
        let mut capabilitySid = BStr::empty();
        assert_eq!(errorInfo.GetErrorDetails(out(&mut description), &mut error, out(&mut restrictedDescription), out(&mut capabilitySid)), S_OK);
        println!("Got Error Info: {} ({})", description, restrictedDescription);
        assert_eq!(error, hres); // the returned HRESULT within IRestrictedErrorInfo is the same as the original HRESULT
        // NOTE: asyncOp is still null pointer at this point
    }
    
    //let hres = deviceInformationStatics.FindAllAsyncAqsFilter(deviceSelector.get(), out(&mut asyncOp));
    assert_eq!(deviceInformationStatics.FindAllAsync(out(&mut asyncOp)), S_OK);
    
    println!("CLS: {}",  asyncOp.get_runtime_class_name());
    
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
    
    let mut status = std::mem::uninitialized();
    let hres = asi.get_Status(&mut status);
    println!("HRESULT (get_Status) = {:x}", hres);
    println!("status: {:?}", status);
    
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    {
        let pair2 = pair.clone();
        let mut myHandler = AsyncOperationCompletedHandler::new(move |_op, status| {
            println!("Result handler invoked! Status: {:?}", status);
            let &(ref lock, ref cvar) = &*pair2;
            let mut started = lock.lock().unwrap();
            *started = true;
            cvar.notify_one();
            S_OK
        });
        assert!(asyncOp.put_Completed(&mut *myHandler) == S_OK);
        // local reference to myHandler is dropped here -> Release() is called
    }
    
    println!("Waiting for results of async call ...");
    
    // use condvar to wait until handler has been called
    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    let mut deviceInformationCollection = ComPtr::uninitialized();
    assert!(asyncOp.GetResults(out(&mut deviceInformationCollection)) == S_OK);
    println!("CLS: {}", deviceInformationCollection.get_runtime_class_name());
    let mut count = 0;
    assert!(deviceInformationCollection.get_Size(&mut count) == S_OK);
    println!("Device Count: {}", count);
    
    let mut remember = None;
    let mut i = 0;
    for mut current in &mut deviceInformationCollection { // without the `&mut` it would consume the ComPtr
        let mut deviceName = HString::empty();
        current.get_Name(out(&mut deviceName));
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
    assert!(deviceInformationCollection.IndexOf(&mut *remember.unwrap(), &mut index, &mut found) == S_OK);
    println!("Found remembered value: {} (index: {})", found == TRUE, index);
    
    let mut get_at_result = ComPtr::uninitialized();
    let hres = deviceInformationCollection.GetAt(2000, out(&mut get_at_result));
    println!("HRESULT (GetAt) = {:x}", hres); // will be E_BOUNDS (out of bounds)
    if hres != S_OK {
        std::mem::forget(get_at_result);
    }
    
    let hres = asi.get_Status(&mut status);
    println!("HRESULT (get_Status) = {:x}", hres);
    println!("status: {:?}", status);
    
    let hres = asi.Close();
    println!("HRESULT (Close AsyncInfo) = {:x}", hres);
}