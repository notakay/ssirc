use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str::{from_utf8};
use std::sync::{mpsc};
use std::thread;

use crate::threadpool::{ThreadPool};

pub fn run_server(port: usize) {
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(addr).unwrap();
    let pool = ThreadPool::new(2);
    let (sender, receiver) = mpsc::channel();

    thread::spawn( || watcher(receiver));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let sender = mpsc::Sender::clone(&sender);
        pool.execute ( || handle_connection(stream, sender) );
    }
}

fn watcher(receiver: mpsc::Receiver<String>) {
    for received in receiver {
        print!("> {}", received);
    }
}

fn handle_connection(mut stream: TcpStream, sender: mpsc::Sender<String>) {
    let mut buf = [0; 128];
    loop {
        let size = stream.read(&mut buf).unwrap();
        let message = from_utf8(&buf[0..size]).unwrap().to_string();
        sender.send(message).unwrap();
        stream.write(&buf[0..size]).unwrap();
        buf = [0; 128];
    }
}