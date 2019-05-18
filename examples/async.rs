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
use winrt::windows::media::*;
use winrt::windows::storage::StorageFile;

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
    let mut async_op = DeviceInformation::find_all_async().unwrap().fuse();

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

    print!("Transcoding media: ");
    let source = StorageFile::get_file_from_path_async(&*FastHString::new("D:\\Desktop\\test-transcode\\test.mp3")).unwrap().await.expect("get_file_from_path_async failed").unwrap();
    let dest = StorageFile::get_file_from_path_async(&*FastHString::new("D:\\Desktop\\test-transcode\\test.flac")).unwrap().await.expect("get_file_from_path_async failed").unwrap();
    let profile = mediaproperties::MediaEncodingProfile::create_flac(mediaproperties::AudioEncodingQuality::Medium).unwrap().unwrap();
    let transcoder = transcoding::MediaTranscoder::new();
    let prepared = transcoder.prepare_file_transcode_async(&source, &dest, &profile).unwrap().await.unwrap().unwrap();
    assert!(prepared.get_can_transcode().unwrap());

    let mut interval = Interval::new(Duration::from_millis(100));
    let async_op = prepared.transcode_async().unwrap();
    // TODO: there should be a way to await the created AsyncActionProgressHandler (and any other delegate) as an async stream
    async_op.set_progress(&AsyncActionProgressHandler::new(|_info, progress| {
        print_progress(progress);
        Ok(())
    })).unwrap();
    let mut async_op = async_op.fuse();

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

    work.await
}

fn print_progress(progress: f64) {
    use std::io::Write;
    print!("{:.0}%", progress);
    std::io::stdout().flush().unwrap();
}
