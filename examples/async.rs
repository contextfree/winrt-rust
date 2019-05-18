#![feature(async_await)]

extern crate winapi;
extern crate winrt;
extern crate futures;

use futures::executor::block_on;

use winrt::*;
use winrt::windows::foundation::*;
use winrt::windows::devices::enumeration::*;
use winrt::windows::devices::midi::*;

fn main() {
    let rt = RuntimeContext::init();
    block_on(run());
    rt.uninit();
}

async fn run() {
    let device_selector = MidiOutPort::get_device_selector().unwrap();
    println!("{}", device_selector);

    let async_op = DeviceInformation::find_all_async().unwrap();
    
    println!("CLS: {}",  async_op.get_runtime_class_name());
    
    let asi = async_op.query_interface::<IAsyncInfo>().unwrap();
    println!("IAsyncInfo: {:p}, Iasync_operation: {:p}", asi, async_op);
    
    let unknown = async_op.query_interface::<IUnknown>().unwrap();
    println!("IAsyncInfo: {:p}, Iasync_operation: {:p}, IUnknown: {:p}", asi, async_op, unknown);
    
    let unknown = asi.query_interface::<IUnknown>().unwrap();
    println!("IAsyncInfo: {:p}, Iasync_operation: {:p}, IUnknown: {:p}", asi, async_op, unknown);
    
    let id = asi.get_id().unwrap();
    println!("id: {:?}", id);
    let status = asi.get_status().unwrap();
    println!("status: {:?}", status);

    let device_information_collection = (&*async_op).await.unwrap().unwrap();
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

    if let Some(mut r) = remember {
        let (index, found) = device_information_collection.index_of(&mut r).unwrap();
        println!("Found remembered value: {} (index: {})", found, index);
        assert_eq!(index, 100);
    }
}
