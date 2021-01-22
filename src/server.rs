use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str::{from_utf8};
use std::sync::{Arc, Mutex, mpsc};
use std::{thread};

use bus::{Bus, BusReader};

use crate::threadpool::{ThreadPool};

pub struct Server {
    relay: Relay,
}

impl Server {
    pub fn new() -> Server {
        let relay = Relay::new();
        Server { relay }
    }

    pub fn run_server(&mut self, port: usize) {
        let addr = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(addr).unwrap();
        let pool = ThreadPool::new(10);

        let (bus_rx_handlers, mpsc_tx_handlers) = self.relay.get_handlers();
        let relay_tx = self.relay.get_signal_tx();

        thread::spawn(move || {
            for stream in listener.incoming() {
                // signal relay to generate handlers
                relay_tx.send(Signal::GenerateBusRx).unwrap();
                relay_tx.send(Signal::GenerateMpscTx).unwrap();

                if let Some(bus_rx) = bus_rx_handlers.lock().unwrap().pop() {
                    if let Some(mpsc_tx) = mpsc_tx_handlers.lock().unwrap().pop() {
                        pool.execute(|| Server::handle_connection(stream.unwrap(), mpsc_tx, bus_rx));
                    }
                }


            }
        });

        self.relay.run_relay();
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

enum Signal {
    GenerateBusRx,
    GenerateMpscTx,
}

struct Relay {
    bus_tx: Bus<String>,
    mpsc_rx: mpsc::Receiver<String>,
    mpsc_tx: mpsc::Sender<String>,
    bus_rx_handlers: Arc<Mutex<Vec<BusReader<String>>>>,
    mpsc_tx_handlers: Arc<Mutex<Vec<mpsc::Sender<String>>>>,
    signal_rx: mpsc::Receiver<Signal>,
    signal_tx: mpsc::Sender<Signal>,
}

impl Relay {
    fn new() -> Relay {
        let bus_tx = Bus::new(10);
        let (mpsc_tx, mpsc_rx) = mpsc::channel();
        let bus_rx_handlers = Vec::with_capacity(10);
        let mpsc_tx_handlers = Vec::with_capacity(10);
        let (signal_tx, signal_rx) = mpsc::channel();

        let bus_rx_handlers = Arc::new(Mutex::new(bus_rx_handlers));
        let mpsc_tx_handlers = Arc::new(Mutex::new(mpsc_tx_handlers));
        Relay {bus_tx, mpsc_rx, mpsc_tx, bus_rx_handlers, mpsc_tx_handlers, signal_rx, signal_tx }
    }

    fn get_handlers(&mut self) -> (Arc<Mutex<Vec<BusReader<String>>>>, Arc<Mutex<Vec<mpsc::Sender<String>>>>) {
        (Arc::clone(&self.bus_rx_handlers), Arc::clone(&self.mpsc_tx_handlers))
    }

    fn get_signal_tx(&self) -> mpsc::Sender<Signal> {
        self.signal_tx.clone()
    }

    fn run_relay(&mut self) {
        loop {
            let message = self.mpsc_rx.try_recv();
            match message {
                Ok(message) => { self.bus_tx.broadcast(message) },
                Err(_) => {},
            }
            if let Ok(_) = self.signal_rx.try_recv() {
                self.bus_rx_handlers.lock().unwrap().push(self.bus_tx.add_rx());
                self.mpsc_tx_handlers.lock().unwrap().push(self.mpsc_tx.clone());    
            }
            
        }
    }
}