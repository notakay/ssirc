use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str::{from_utf8};
use std::sync::{mpsc};
use std::{thread, time};

use bus::{Bus, BusReader};

use crate::threadpool::{ThreadPool};

pub struct Server;

impl Server {
    pub fn run_server(port: usize) {
        let addr = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(addr).unwrap();
        let pool = ThreadPool::new(2);
        let (sender, receiver) = mpsc::channel();

        let mut bus = Bus::new(10);
        let mut bus_rx_handlers = Vec::with_capacity(10);
        for _ in 0..10 {
            bus_rx_handlers.push(bus.add_rx());
        }
        
        thread::spawn( move || Server::watcher(receiver, bus));

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let sender = mpsc::Sender::clone(&sender);
            if let Some(bus_rx) = bus_rx_handlers.pop() {
                pool.execute ( || Server::handle_connection(stream, sender, bus_rx) );
            }
        }
    }

    fn watcher(receiver: mpsc::Receiver<String>, mut bus_tx: Bus<String>) {
        for received in receiver {
            thread::sleep(time::Duration::from_secs(2));
            print!("Broadcasting {}", received);
            bus_tx.broadcast(received);
        }
    }

    fn handle_connection(mut stream: TcpStream, sender: mpsc::Sender<String>, mut bus_rx: BusReader<String>) {
        let mut buf = [0; 128];
        loop {
            let size = stream.read(&mut buf).unwrap();
            let message = from_utf8(&buf[0..size]).unwrap().to_string();
            sender.send(message).unwrap();
            
            let message = bus_rx.recv().unwrap();
            stream.write(message.as_bytes()).unwrap();
            //stream.write(&buf[0..size]).unwrap();
            buf = [0; 128];
        }
    }
}