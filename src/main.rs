use std::env;

mod client;
mod server;
mod helper;

use crate::client::{run_client};
use crate::server::{run_server};
use crate::helper::{setopts, parse_args, Mode};


fn main() {
    let args: Vec<String> = env::args().collect();
    let opts = setopts();
    let mode = parse_args(&args, opts);

    match mode {
        Mode::Server(port) => run_server(port),
        Mode::Client(host, port) => run_client(host, port)
    }
}
