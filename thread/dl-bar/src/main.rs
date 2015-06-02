extern crate hyper;

use std::io::prelude::*;
use std::io::BufReader;
use std::thread;

use hyper::Client;
use hyper::header::ContentLength;


fn main() {

    let mut client = Client::new();

    //let mut response = client.get("http://static.rust-lang.org/dist/rust-1.0.0-x86_64-pc-windows-gnu.msi")
    let response = client.get("http://static2.mangainn.me/mangas/324/8837/01_10_10_2011_23_31_31.jpg")
        .send().unwrap();

    let mut content_length: usize  = 0;
    {
        let ContentLength(length) = *response.headers.get::<ContentLength>().unwrap();
        content_length = length as usize;
    }
    println!("Content length is {}", content_length);

    let mut buffer = [0u8; 1024];
    let mut read_size = 0;

    let mut reader = BufReader::new(response);
    loop {
        let read_result = reader.read(&mut buffer);
        match read_result {
            Ok(0) => break,
            Err(_) => break,
            Ok(size) => {
                read_size += size;
                let percent: f32 = (read_size as f32 / content_length as f32) * 100.0;
                println!("Completed {:.2}% ({}/{})", percent, read_size, content_length);
            }
        }
    }
}
