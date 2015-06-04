extern crate hyper;

use std::io::prelude::*;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::thread;

use hyper::Client;
use hyper::header::ContentLength;


fn main() {

    let mut client = Client::new();

    //let mut response = client.get("http://static.rust-lang.org/dist/rust-1.0.0-x86_64-pc-windows-gnu.msi")
    //let response = client.get("http://static2.mangainn.me/mangas/324/8837/01_10_10_2011_23_31_31.jpg")
    let response = client.get("http://speedtest.singapore.linode.com/100MB-singapore.bin")
        .send().unwrap();

    let mut content_length: usize;
    {
        let ContentLength(length) = *response.headers.get::<ContentLength>().unwrap();
        content_length = length as usize;
    }
    println!("Content length is {}", content_length);

    let read_size = Arc::new(Mutex::new(0));

    let shared_read_size = read_size.clone();


    let download_handle = thread::spawn(move || {

        let mut reader = BufReader::new(response);
        loop {

            let mut buffer = [0u8; 1024];
            let read_result = reader.read(&mut buffer);
            match read_result {
                Ok(0) => break,
                Err(_) => break,
                Ok(size) => {
                    let mut read_size = shared_read_size.lock().unwrap();
                    *read_size += size;
                }
            }
        }

    });

    let shared_read_size = read_size.clone();
    let gui_handle = thread::spawn(move || {
        loop {
            let mut size;
            {
                let read_size = shared_read_size.lock().unwrap();
                size = *read_size as f32
            }
            let percent: f32 = (size / content_length as f32) * 100.0;
            print!("\rCompleted {:.2}% ({}/{}) ", percent, size, content_length);

            if percent == 100f32 {
                break;
            }
            thread::sleep_ms(100);
        }
    });

    download_handle.join().unwrap();
    gui_handle.join().unwrap();

}
