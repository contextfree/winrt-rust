extern crate winapi;
extern crate winrt;

use std::ptr;

use winrt::*;
use winrt::windows::foundation::*;
use winrt::windows::foundation::collections::*;
use winrt::windows::devices::enumeration::*;
use winrt::windows::devices::midi::*;
use winrt::windows::storage::*;

fn main() {
    let base = FastHString::new("https://github.com");
    let relative = FastHString::new("contextfree/winrt-rust");
    let uri = Uri::create_with_relative_uri(&base, &relative).unwrap();
    let uri2 = uri.clone();
    let to_string = uri.query_interface::<IStringable>().unwrap().to_string().unwrap();
    println!("{} -> {}", uri.get_runtime_class_name(), to_string);
    println!("TrustLevel: {:?}", uri.get_trust_level());
    println!("GetIids:");
    let iids = uri.get_iids();
    for i in 0..iids.len() {
        println!("  [{}] = {:?}", i, iids[i]);
    }

    //let mut out_port_statics = MidiOutPort::get_activation_factory();
    //println!("out_port_statics: {}", out_port_statics.get_runtime_class_name()); // this is not allowed (prevented statically)
    
    let device_selector = MidiOutPort::get_device_selector().unwrap();
    println!("{}", device_selector);
    
    unsafe {
        use winapi::shared::winerror::S_OK;
        use winapi::winrt::roerrorapi::GetRestrictedErrorInfo;

        // Test some error reporting by using an invalid device selector
        let wrong_deviceselector: FastHString = "Foobar".into();
        let res = DeviceInformation::find_all_async_aqs_filter(&wrong_deviceselector);
        if let Err(e) = res {
            println!("HRESULT (FindAllAsyncAqsFilter) = {:?}", e);
            // let error_info = {
            //     let mut res = ptr::null_mut();
            //     assert_eq!(GetRestrictedErrorInfo(&mut res), S_OK);
            //     ComPtr::wrap_nonnull(res)
            // };
            // let (description, error, restricted_description, _) = {
            //     let mut description = ptr::null_mut();
            //     let mut error = 0;
            //     let mut restricted_description = ptr::null_mut();
            //     let mut capability_sid = ptr::null_mut();
            //     assert_eq!(error_info.as_abi().GetErrorDetails(&mut description, &mut error, &mut restricted_description, &mut capability_sid), S_OK);
            //     (BStr::wrap(description), error, BStr::wrap(restricted_description), BStr::wrap(capability_sid))
            // };
            // println!("Got Error Info: {} ({})", description, restricted_description);
            // assert_eq!(error, e.as_hresult()); // the returned HRESULT within IRestrictedErrorInfo is the same as the original HRESULT
        }
        // NOTE: `res` is still null pointer at this point
    };

    let async_op = DeviceInformation::find_all_async().unwrap();
    
    println!("CLS: {}",  async_op.get_runtime_class_name());
    
    let asi = async_op.query_interface::<IAsyncInfo>().unwrap();
    println!("IAsyncInfo: {:p}, Iasync_operation: {:p}", asi.get_abi(), async_op.get_abi());
    
    let unknown = async_op.query_interface::<IUnknown>().unwrap();
    println!("IAsyncInfo: {:p}, Iasync_operation: {:p}, IUnknown: {:p}", asi.get_abi(), async_op.get_abi(), unknown.get_abi());
    
    let unknown = asi.query_interface::<IUnknown>().unwrap();
    println!("IAsyncInfo: {:p}, Iasync_operation: {:p}, IUnknown: {:p}", asi.get_abi(), async_op.get_abi(), unknown.get_abi());
    
    let id = asi.get_id().unwrap();
    println!("id: {:?}", id);
    let status = asi.get_status().unwrap();
    println!("status: {:?}", status);

    let device_information_collection = async_op.blocking_get().unwrap().unwrap();
    println!("CLS: {}", device_information_collection.get_runtime_class_name());
    let count = device_information_collection.get_size().unwrap();
    println!("Device Count: {}", count);
    
    let mut remember = None;
    let mut i = 0;
    for current in device_information_collection.into_iter() {
        let current = current.expect("current was null");
        let device_name = current.get_name().unwrap();
        println!("Device Name ({}): {}", i, device_name);
        if i == 100 {
            // remember the 100th value and try to find it later using IndexOf
            remember = Some(current);
        }
        i += 1;
    }
    assert_eq!(i, count);

    let mut buffer = Vec::with_capacity(2000);
    device_information_collection.get_many(0, &mut buffer).unwrap();
    for (b, i) in buffer.iter().zip(0..) {
        let device_name = b.get_name().unwrap();
        println!("Device Name ({}): {}", i, device_name);
    }
    let len = buffer.len();
    drop(buffer);
    println!("Freed result of GetMany ({} values).", len);

    if let Some(mut r) = remember {
        let (index, found) = device_information_collection.index_of(&mut r).unwrap();
        println!("Found remembered value: {} (index: {})", found, index);
        assert_eq!(index, 100);
    }
    
    match device_information_collection.get_at(count + 42) {
        Err(e) => println!("Error getting element at {}: {:?}", count + 42, e), // will be out of bounds
        Ok(_) => panic!("expected Error")
    };

    let array = &[true, false, false, true];
    let boxed_array = PropertyValue::create_boolean_array(array).unwrap().unwrap();
    let boxed_array = boxed_array.query_interface::<IPropertyValue>().unwrap();
    assert_eq!(boxed_array.get_type().unwrap(), PropertyType::BooleanArray);
    let boxed_array = boxed_array.query_interface::<IReferenceArray<bool>>().unwrap();
    let returned_array = boxed_array.get_value().unwrap();
    println!("{:?} = {:?}", array, &returned_array[..]);
    assert_eq!(array, &returned_array[..]);

    let str1 = FastHString::new("foo");
    let str2 = FastHString::new("bar");
    let array: &[&HStringArg] = &[&str1, &str2, &str1, &str2];
    let boxed_array = PropertyValue::create_string_array(array).unwrap().unwrap();
    let boxed_array = boxed_array.query_interface::<IPropertyValue>().unwrap();
    assert_eq!(boxed_array.get_type().unwrap(), PropertyType::StringArray);
    let boxed_array = boxed_array.query_interface::<IReferenceArray<HString>>().unwrap();
    let returned_array = boxed_array.get_value().unwrap();
    assert_eq!(array.len(), returned_array.len());
    for i in 0..array.len() {
        assert!(returned_array[i] == (if i % 2 == 0 { &str1 } else { &str2 }));
    }
    // TODO: test array interface objects (also see if ComArray drops contents correctly)
    
    let status = asi.get_status().unwrap();
    println!("status: {:?}", status);
    
    assert!(asi.close().is_ok());

    // Walk directories up to root
    let exe_path = std::env::current_exe().expect("current_exe failed");
    let exe_path_str = exe_path.to_str().expect("invalid unicode path");
    let file = StorageFile::get_file_from_path_async(&FastHString::new(&exe_path_str)).unwrap().blocking_get().expect("get_file_from_path_async failed").unwrap();
    println!("Executable file: {}", file.query_interface::<IStorageItem>().unwrap().get_path().unwrap());
    /*let mut parent = file.query_interface::<IStorageItem>().unwrap();
    loop {
        parent = parent.query_interface::<IStorageItem2>().unwrap().get_parent_async().unwrap().blocking_get().unwrap().unwrap().query_interface::<IStorageItem>().unwrap();
        println!("Parent: {}", parent.get_path().unwrap());
        // ... until parent == null, but this currently does not work because we don't support methods returning null
    }*/
    let exe_folder = file.query_interface::<IStorageItem2>().unwrap().get_parent_async().unwrap().blocking_get().expect("get_parent_async failed").unwrap();    
    let txt_file = exe_folder.create_file_async(&FastHString::new("__test_file.txt"), CreationCollisionOption::ReplaceExisting).unwrap().blocking_get().expect("create_file_async failed").unwrap();
    println!("Created text file {}", txt_file.query_interface::<IStorageItem>().unwrap().get_path().unwrap());
    FileIO::append_text_async(&txt_file, &FastHString::new("This is a test\nand a second line.")).unwrap().blocking_wait();
    let mut lines: IVector<HString> = FileIO::read_lines_async(&txt_file).unwrap().blocking_get().expect("read_lines_async failed").unwrap();
    println!("Read {} lines from the text file", lines.get_size().expect("get_size failed"));
    // now we have an IVector that we can mess with
    lines.append(&FastHString::new("The third line, added later")).expect("append failed");
    assert_eq!(lines.get_size().unwrap(), 3);
    println!("=== Lines: ===");
    for line in &lines {
        println!("{}", line);
        // the following won't compile (iterator invalidation):
        //lines.remove_at(0).expect("remove_at failed");
    }
    println!("=== Guid test ===");
    let new_guid = GuidHelper::create_new_guid().unwrap();
    println!("create_new_guid -> {:?}", new_guid);
    let empty_guid = GuidHelper::get_empty().unwrap();
    println!("get_empty -> {:?}", empty_guid);
    let empty_guid2 = GuidHelper::get_empty().unwrap();
    assert!(!GuidHelper::equals(&empty_guid, &new_guid).unwrap());
    assert!(GuidHelper::equals(&empty_guid, &empty_guid2).unwrap());
}
