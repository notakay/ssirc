use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str::{from_utf8};

pub fn run_client(ip: String, port: usize) {
    let addr = format!("{}:{}", ip, port);
    let mut stream = TcpStream::connect(addr).unwrap();
    let mut buf = [0; 128];

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        stream.write(input.as_bytes()).unwrap();

        let size = stream.read(&mut buf).unwrap();
        print!("> {}", from_utf8(&buf[0..size]).unwrap());
        buf = [0; 128];
    }
}