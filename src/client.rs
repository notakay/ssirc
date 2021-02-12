use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str::{from_utf8};
use std::thread;

pub struct Client;

impl Client {
    pub fn run_client(ip: String, port: usize) {
        let addr = format!("{}:{}", ip, port);
        let mut stream = TcpStream::connect(addr).unwrap();
        
        {
            let mut stream = stream.try_clone().unwrap();
            let mut buf = [0; 128];
            thread::spawn(move || loop {
                let size = stream.read(&mut buf).unwrap();
                let message = from_utf8(&buf[0..size]).unwrap();
                print!("{}", message);
                buf = [0; 128];
            });
        }

        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            stream.write(input.as_bytes()).unwrap();
        }
    }
}