extern crate winapi as w;
extern crate winrt as wrt;
// TODO: don't use functions from runtimeobject directly 
extern crate runtimeobject;

use std::ptr;

use wrt::*;
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

    let mut uri_factory: ComPtr<IUriRuntimeClassFactory> = Uri::get_activation_factory();
    let base = FastHString::new("https://github.com");
    let relative = FastHString::new("contextfree/winrt-rust");
    let uri = unsafe { uri_factory.create_with_relative_uri(&base, &relative).unwrap() };
    let to_string = unsafe { uri.query_interface::<IStringable>().unwrap().to_string().unwrap() };
    println!("{} -> {}", uri.get_runtime_class_name(), to_string);
    println!("TrustLevel: {:?}", uri.get_trust_level());
    println!("GetIids:");
    let iids = uri.get_iids();
    for i in 0..iids.len() {
        println!("  [{}] = {:?}", i, iids[i]);
    }

    let mut out_port_statics = MidiOutPort::get_activation_factory();
    //println!("out_port_statics: {}", out_port_statics.get_runtime_class_name()); // this is not allowed (prevented statically)
    
    let device_selector = unsafe { out_port_statics.get_device_selector().unwrap() };
    println!("{}", device_selector);
    
    let mut device_information_statics: ComPtr<IDeviceInformationStatics> = DeviceInformation::get_activation_factory();
    
    unsafe {
        use runtimeobject::*;
        use ::w::S_OK;

        // Test some error reporting by using an invalid device selector
        let wrong_deviceselector: FastHString = "Foobar".into();
        let res = device_information_statics.find_all_async_aqs_filter(&wrong_deviceselector);
        if let Err(e) = res {
            println!("HRESULT (FindAllAsyncAqsFilter) = {:?}", e.as_hresult());
            let mut error_info = {
                let mut res = ptr::null_mut();
                assert_eq!(GetRestrictedErrorInfo(&mut res), S_OK);
                ComPtr::wrap(res)
            };
            let (description, error, restricted_description, _) = {
                let mut description = ptr::null_mut();
                let mut error = 0;
                let mut restricted_description = ptr::null_mut();
                let mut capability_sid = ptr::null_mut();
                assert_eq!(error_info.GetErrorDetails(&mut description, &mut error, &mut restricted_description, &mut capability_sid), S_OK);
                (BStr::wrap(description), error, BStr::wrap(restricted_description), BStr::wrap(capability_sid))
            };
            println!("Got Error Info: {} ({})", description, restricted_description);
            assert_eq!(error, e.as_hresult()); // the returned HRESULT within IRestrictedErrorInfo is the same as the original HRESULT
        }
        // NOTE: `res` is still null pointer at this point
    };

    //let mut async_op = unsafe { device_information_statics.find_all_async_aqs_filter(device_selector.get_ref()).unwrap() };
    let mut async_op = unsafe { device_information_statics.find_all_async().unwrap() };
    
    println!("CLS: {}",  async_op.get_runtime_class_name());
    
    let mut asi = async_op.query_interface::<IAsyncInfo>().unwrap();
    println!("IAsyncInfo: {:p}, Iasync_operation: {:p}", asi, async_op);
    
    let unknown = async_op.query_interface::<IUnknown>().unwrap();
    println!("IAsyncInfo: {:p}, Iasync_operation: {:p}, IUnknown: {:p}", asi, async_op, unknown);
    
    let unknown = asi.query_interface::<IUnknown>().unwrap();
    println!("IAsyncInfo: {:p}, Iasync_operation: {:p}, IUnknown: {:p}", asi, async_op, unknown);
    
    let id = unsafe { asi.get_id().unwrap() };
    println!("id: {:?}", id);
    
    let status = unsafe { asi.get_status().unwrap() };
    println!("status: {:?}", status);
    
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    {
        let pair2 = pair.clone();
        let mut my_handler = AsyncOperationCompletedHandler::new(move |_op, status| {
            println!("Result handler invoked! Status: {:?}", status);
            let &(ref lock, ref cvar) = &*pair2;
            let mut started = lock.lock().unwrap();
            *started = true;
            cvar.notify_one();
            Ok(())
        });
        unsafe { async_op.set_completed(&mut my_handler).unwrap() };
        // local reference to my_handler is dropped here -> Release() is called
    }
    
    println!("Waiting for results of async call ...");
    
    // use condvar to wait until handler has been called
    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    let mut device_information_collection = unsafe { async_op.get_results().unwrap() };
    println!("CLS: {}", device_information_collection.get_runtime_class_name());
    let count = unsafe { device_information_collection.get_size().unwrap() };
    println!("Device Count: {}", count);
    
    let mut remember = None;
    let mut i = 0;
    for mut current in device_information_collection.into_iter() {
        let device_name = unsafe { current.get_name().unwrap() };
        println!("Device Name ({}): {}", i, device_name);
        if i == 100 {
            // remember the 100th value and try to find it later using IndexOf
            remember = Some(current);
        }
        i += 1;
    }
    assert_eq!(i, count);

    let mut buffer = Vec::with_capacity(2000);
    unsafe { device_information_collection.get_many(0, &mut buffer).unwrap() };
    for (b, i) in buffer.iter_mut().zip(0..) {
        let device_name = unsafe { b.get_name().unwrap() };
        println!("Device Name ({}): {}", i, device_name);
    }
    let len = buffer.len();
    drop(buffer);
    println!("Freed result of GetMany ({} values).", len);

    if let Some(mut r) = remember {
        let (index, found) = unsafe { device_information_collection.index_of(&mut r).unwrap() };
        println!("Found remembered value: {} (index: {})", found, index);
    }
    
    match unsafe { device_information_collection.get_at(count + 42) } {
        Err(e) => println!("Error getting element at {}: {:?}", count + 42, e), // will be out of bounds
        Ok(_) => panic!("expected Error")
    };

    let array = &mut [true, false, false, true];
    let boxed_array = unsafe { PropertyValue::get_activation_factory().create_boolean_array(array) };
    let mut boxed_array = boxed_array.unwrap().query_interface::<IPropertyValue>().unwrap();
    assert_eq!(unsafe { boxed_array.get_type().unwrap() }, PropertyType_BooleanArray);
    let mut boxed_array = boxed_array.query_interface::<IReferenceArray<bool>>().unwrap();
    let returned_array = unsafe { boxed_array.get_value().unwrap() };
    println!("{:?} = {:?}", array, &returned_array[..]);
    assert_eq!(array, &returned_array[..]);

    let str1 = FastHString::new("foo");
    let str2 = FastHString::new("bar");
    let array = &mut [&*str1, &*str2, &*str1, &*str2];
    let boxed_array = unsafe { PropertyValue::get_activation_factory().create_string_array(array) };
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