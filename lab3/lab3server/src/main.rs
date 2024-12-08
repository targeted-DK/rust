//main.rs
//Author : DK Kim, donggyukim@wustl.edu
//main file that runs the "lab3server" program
pub mod lab3;

use lab3::return_wrapper::*;
use lab3::server::Server;
use std::env;
use std::io::Write;

const NETWORK_ADDRESS : usize = 1;
const BAD_ARGS : u8 = 1;
const MIN_ARGS_FOR_SERVER : usize = 2;

fn main() -> ReturnWrapper {
    

    let args: Vec<String> = env::args().collect();

    if args.len() != MIN_ARGS_FOR_SERVER {
        usage();
        return ReturnWrapper::new(BAD_ARGS.into());
    }

    let network_address = args[NETWORK_ADDRESS].clone();

    let mut server = Server::new();

    server.open(&network_address);

    server.run();


    ReturnWrapper::new(SUCCESS.into())
}


fn usage() {
    let mut writer = std::io::stdout().lock();

    let _ = writeln!(&mut writer, "Running a server needs exactly two arguments, which are program name(lab3server) and a network address");
    let _ = writeln!(&mut writer, "usage: ./lab3server <network address>");

}