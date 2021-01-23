use std::env;

mod client;
mod server;
mod helper;
mod threadpool;

use crate::client::{Client};
use crate::server::{Server};
use crate::helper::{setopts, parse_args, Mode};


fn main() {
    let args: Vec<String> = env::args().collect();
    let opts = setopts();
    let mode = parse_args(&args, opts);

    match mode {
        Mode::Server(port) => {
            let mut server = Server::new();
            server.run_server(port);
            server.run_relay();
        },
        Mode::Client(host, port) => Client::run_client(host, port)
    }
}
