#![feature(async_await)]

extern crate winapi;
extern crate winrt;
extern crate futures;

use std::time::Duration;

use futures::executor::block_on;
use futures::prelude::*;
use futures::select;

use futures_native_timers::Interval;

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

    // let async_op = DeviceInformation::find_all_async().unwrap();
    
    // println!("CLS: {}",  async_op.get_runtime_class_name());
    
    // let asi = async_op.query_interface::<IAsyncInfo>().unwrap();
    // println!("IAsyncInfo: {:p}, Iasync_operation: {:p}", asi, async_op);
    
    // let unknown = async_op.query_interface::<IUnknown>().unwrap();
    // println!("IAsyncInfo: {:p}, Iasync_operation: {:p}, IUnknown: {:p}", asi, async_op, unknown);
    
    // let unknown = asi.query_interface::<IUnknown>().unwrap();
    // println!("IAsyncInfo: {:p}, Iasync_operation: {:p}, IUnknown: {:p}", asi, async_op, unknown);
    
    // let id = asi.get_id().unwrap();
    // println!("id: {:?}", id);
    // let status = asi.get_status().unwrap();
    // println!("status: {:?}", status);

    let mut interval = Interval::new(Duration::from_millis(100));
    let mut async_op = DeviceInformation::find_all_async().unwrap().fuse(); // TODO: get rid of fuse()

    let work = async {
        let mut result = None;
        loop {
            select! {
                _ = interval.next() => {
                    use std::io::Write;
                    print!(".");
                    std::io::stdout().flush().unwrap();
                },
                res = async_op => {
                    result = Some(res);
                    println!("");
                    break;
                }
            };
        }
        result.unwrap()
    };

    let device_information_collection = work.await.unwrap().unwrap();
    println!("CLS: {}", device_information_collection.get_runtime_class_name());
    let count = device_information_collection.get_size().unwrap();
    println!("Device Count: {}", count);

    for (i, current) in device_information_collection.into_iter().enumerate().take(10) {
        let current = current.expect("current was null");
        let device_name = current.get_name().unwrap();
        println!("Device Name ({}): {}", i, device_name);
    }
}
