use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str::{from_utf8};

pub fn run_server(port: usize) {
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream), // handle on a different thread??
            Err(e) => println!("{:?}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 128];
    loop {
        let size = stream.read(&mut buf).unwrap();
        print!("{}", from_utf8(&buf[0..size]).unwrap());
        stream.write(&buf[0..size]).unwrap();
        buf = [0; 128];
    }
}