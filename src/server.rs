use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str::{from_utf8};
use std::sync::{Arc, Mutex, mpsc};
use std::{thread};

use bus::{Bus, BusReader};

use crate::threadpool::{ThreadPool};

pub struct Server {
    bus_tx: Arc<Mutex<Bus<String>>>,
    mpsc_tx: mpsc::Sender<String>,
    mpsc_rx: mpsc::Receiver<String>,
}

impl Server {
    pub fn new() -> Server {
        let bus_tx = Arc::new(Mutex::new(Bus::new(10)));
        let (mpsc_tx, mpsc_rx) = mpsc::channel();
        Server { bus_tx, mpsc_tx, mpsc_rx }
    }

    pub fn run_server(&mut self, port: usize) {
        let addr = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&addr).unwrap();
        let pool = ThreadPool::new(10);

        let bus_tx = Arc::clone(&self.bus_tx);
        let mpsc_tx = self.mpsc_tx.clone();
        thread::spawn(move || {
            let mut client_id = 1;
            for stream in listener.incoming() {
                let mpsc_tx = mpsc_tx.clone();
                let bus_rx = bus_tx.lock().unwrap().add_rx();
                {
                    let client_id = client_id.clone();
                    pool.execute(move || Server::handle_connection(stream.unwrap(), mpsc_tx, bus_rx, client_id));
                }
                client_id = client_id + 1;
            }
        });
    }

    pub fn run_relay(&self) {
        loop {
            let message = self.mpsc_rx.try_recv();
            match message {
                Ok(message) => { self.bus_tx.lock().unwrap().broadcast(message) },
                Err(_) => {},
            }
        }
    }

    fn handle_connection(mut stream: TcpStream, sender: mpsc::Sender<String>, mut bus_rx: BusReader<String>, client_id: u32) {
        let mut buf = [0; 128];

        {
            let mut stream = stream.try_clone().unwrap();
            thread::spawn (move || loop {
                let message = bus_rx.recv().unwrap();
                stream.write(message.as_bytes()).unwrap();
                
            });
        }

        loop {
            let size = stream.read(&mut buf).unwrap();
            let message = from_utf8(&buf[0..size]).unwrap().to_string();
            let message = format!("[Client {}] {}", client_id, message);
            sender.send(message).unwrap();
            buf = [0; 128];
        }
    }
}