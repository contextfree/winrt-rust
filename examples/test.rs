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
    S_OK,
};

use wrt::windows::foundation::*;
use wrt::windows::devices::enumeration::*;
use wrt::windows::devices::midi::*;

fn main() {
    let rt = RuntimeContext::init();
    run();
    rt.uninit();
}

fn run() {
    use std::sync::{Arc, Mutex, Condvar};

    let mut uriFactory = Uri::factory();
    let base = FastHString::new("https://github.com");
    let relative = FastHString::new("contextfree/winrt-rust");
    let uri = unsafe { uriFactory.create_with_relative_uri(&base, &relative).unwrap() };
    let to_string = unsafe { uri.query_interface::<IStringable>().unwrap().to_string().unwrap() };
    println!("{} -> {}", uri.get_runtime_class_name(), to_string);
    println!("TrustLevel: {:?}", uri.get_trust_level());
    println!("GetIids:");
    let iids = uri.get_iids();
    for i in 0..iids.len() {
        println!("  [{}] = {:?}", i, iids[i]);
    }

    let mut outPortStatics = IMidiOutPortStatics::factory();
    //println!("outPortStatics: {}", outPortStatics.get_runtime_class_name()); // this is not allowed (prevented statically)
    
    let deviceSelector = unsafe { outPortStatics.get_device_selector().unwrap() };
    println!("{}", deviceSelector);
    
    let mut deviceInformationStatics = IDeviceInformationStatics::factory();
    
    unsafe {
        // Test some error reporting by using an invalid device selector
        let wrongDeviceSelector: FastHString = "Foobar".into();
        let res = deviceInformationStatics.find_all_async_aqs_filter(&wrongDeviceSelector);
        if let Err(e) = res {
            println!("HRESULT (FindAllAsyncAqsFilter) = {:?}", e.as_hresult());
            let mut errorInfo = {
                let mut res = ptr::null_mut();
                assert_eq!(GetRestrictedErrorInfo(&mut res), S_OK);
                ComPtr::wrap(res)
            };
            let (description, error, restrictedDescription, _) = {
                let mut description = ptr::null_mut();
                let mut error = 0;
                let mut restrictedDescription = ptr::null_mut();
                let mut capabilitySid = ptr::null_mut();
                assert_eq!(errorInfo.GetErrorDetails(&mut description, &mut error, &mut restrictedDescription, &mut capabilitySid), S_OK);
                (BStr::wrap(description), error, BStr::wrap(restrictedDescription), BStr::wrap(capabilitySid))
            };
            println!("Got Error Info: {} ({})", description, restrictedDescription);
            assert_eq!(error, e.as_hresult()); // the returned HRESULT within IRestrictedErrorInfo is the same as the original HRESULT
        }
        // NOTE: `res` is still null pointer at this point
    };

    //let mut asyncOp = unsafe { deviceInformationStatics.find_all_async_aqs_filter(deviceSelector.get_ref()).unwrap() };
    let mut asyncOp = unsafe { deviceInformationStatics.find_all_async().unwrap() };
    
    println!("CLS: {}",  asyncOp.get_runtime_class_name());
    
    let mut asi = asyncOp.query_interface::<IAsyncInfo>().unwrap();
    println!("IAsyncInfo: {:p}, IAsyncOperation: {:p}", asi, asyncOp);
    
    let unknown = asyncOp.query_interface::<IUnknown>().unwrap();
    println!("IAsyncInfo: {:p}, IAsyncOperation: {:p}, IUnknown: {:p}", asi, asyncOp, unknown);
    
    let unknown = asi.query_interface::<IUnknown>().unwrap();
    println!("IAsyncInfo: {:p}, IAsyncOperation: {:p}, IUnknown: {:p}", asi, asyncOp, unknown);
    
    let id = unsafe { asi.get_id().unwrap() };
    println!("id: {:?}", id);
    
    let status = unsafe { asi.get_status().unwrap() };
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
            S_OK // TODO: return Result instead
        });
        unsafe { asyncOp.set_completed(&mut myHandler).unwrap() };
        // local reference to myHandler is dropped here -> Release() is called
    }
    
    println!("Waiting for results of async call ...");
    
    // use condvar to wait until handler has been called
    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    let mut deviceInformationCollection = unsafe { asyncOp.get_results().unwrap() };
    println!("CLS: {}", deviceInformationCollection.get_runtime_class_name());
    let count = unsafe { deviceInformationCollection.get_size().unwrap() };
    println!("Device Count: {}", count);
    
    let mut remember = None;
    let mut i = 0;
    for mut current in deviceInformationCollection.into_iter() {
        let deviceName = unsafe { current.get_name().unwrap() };
        println!("Device Name ({}): {}", i, deviceName);
        if i == 100 {
            // remember the 100th value and try to find it later using IndexOf
            remember = Some(current);
        }
        i += 1;
    }
    assert_eq!(i, count);

    let mut buffer = Vec::with_capacity(2000);
    unsafe { deviceInformationCollection.get_many(0, &mut buffer).unwrap() };
    for (b, i) in buffer.iter_mut().zip(0..) {
        let deviceName = unsafe { b.get_name().unwrap() };
        println!("Device Name ({}): {}", i, deviceName);
    }
    let len = buffer.len();
    drop(buffer);
    println!("Freed result of GetMany ({} values).", len);

    if let Some(mut r) = remember {
        let (index, found) = unsafe { deviceInformationCollection.index_of(&mut r).unwrap() };
        println!("Found remembered value: {} (index: {})", found, index);
    }
    
    assert!(unsafe { deviceInformationCollection.get_at(count + 42).is_err() }); // will be E_BOUNDS (out of bounds)

    let array = &mut [true, false, false, true];
    let boxed_array = unsafe { IPropertyValueStatics::factory().create_boolean_array(array) };
    let mut boxed_array = boxed_array.unwrap().query_interface::<IPropertyValue>().unwrap();
    assert_eq!(unsafe { boxed_array.get_type().unwrap() }, PropertyType_BooleanArray);
    let mut boxed_array = boxed_array.query_interface::<IReferenceArray<bool>>().unwrap();
    let returned_array = unsafe { boxed_array.get_value().unwrap() };
    println!("{:?} = {:?}", array, &returned_array[..]);
    assert_eq!(array, &returned_array[..]);

    let str1 = FastHString::new("foo");
    let str2 = FastHString::new("bar");
    let array = &mut [&*str1, &*str2, &*str1, &*str2];
    let boxed_array = unsafe { IPropertyValueStatics::factory().create_string_array(array) };
    let mut boxed_array = boxed_array.unwrap().query_interface::<IPropertyValue>().unwrap();
    assert_eq!(unsafe { boxed_array.get_type().unwrap() }, PropertyType_StringArray);
    let mut boxed_array = boxed_array.query_interface::<IReferenceArray<HString>>().unwrap();
    let returned_array = unsafe { boxed_array.get_value().unwrap() };
    assert_eq!(array.len(), returned_array.len());
    for i in 0..array.len() {
        assert!(returned_array[i] == (if i % 2 == 0 { &str1 } else { &str2 }));
    }
    // TODO: test array interface objects (also see if ComArray drops contents correctly)
    
    let status = unsafe { asi.get_status().unwrap() };
    println!("status: {:?}", status);
    
    assert!(unsafe { asi.close().is_ok() });
}