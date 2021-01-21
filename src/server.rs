use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str::{from_utf8};

use crate::threadpool::{ThreadPool};

pub fn run_server(port: usize) {
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(addr).unwrap();
    let pool = ThreadPool::new(2);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute ( || handle_connection(stream) );
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