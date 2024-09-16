#![allow(unused_imports)]
use std::{
    io::{BufReader, BufWriter},
    net::TcpListener,
    thread,
};

use handlers::handle_request;
use request::RequestParser;
use response::ResponseWriter;

mod handlers;
mod request;
mod response;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    let mut reader = BufReader::new(stream.try_clone().unwrap());
                    let mut writer = BufWriter::new(stream);

                    while let Ok(req) = reader.parse_request() {
                        let res = handle_request(req);
                        match res {
                            Err(e) => println!("error: {}", e),
                            Ok(res) => writer.write_response(res).unwrap(),
                        }
                    }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
