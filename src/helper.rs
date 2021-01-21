extern crate getopts;
use getopts::Options;

pub enum Mode {
    Server(usize),
    Client(String, usize),
}

pub fn setopts() -> Options {
    let mut opts = Options::new();
    opts.optflag("s", "server", "Server mode");
    opts
}

pub fn parse_args(args: &Vec<String>, opts: Options) -> Mode {
    let matches = opts.parse(&args[1..]).unwrap();
    if matches.opt_present("s") {
        Mode::Server(8081) // run on port 8081
    } else {
        Mode::Client(String::from("127.0.0.1"), 8081) // connect to 127.0.0.1:8081
    }
}