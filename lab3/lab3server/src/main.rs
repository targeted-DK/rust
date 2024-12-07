//main.rs
//Author : DK Kim, donggyukim@wustl.edu
//main file that runs the "lab3server" program
pub mod lab3;

use lab3::return_wrapper::*;
use lab3::server::Server;

const PROG_NAME : u8 = 0;
const NETWORK_ADDRESS : u8 = 1;
const BAD_ARGS : u8 = 1;
const MIN_ARGS_FOR_SERVER : u8 = 2;

fn main() -> ReturnWrapper {
    

    //Instruction 11
    let args: Vec<String> = env::args().collect();

    if args.len() != MIN_ARGS_FOR_SERVER {
        usage(&args[PROG_NAME]);
        return ReturnWrapper::new(BAD_ARGS.into());
    }

    let mut program_name = args[PROG_NAME].clone();
    let mut network_address = args[NETWORK_ADDRESS].clone();

    let server = Server::new();

    server.open(network_address);
    server.run();





    ReturnWrapper::new(SUCCESS.into())
}


fn usage(prog_name : &String) {
    let mut writer = std::io::stdout().lock();

    //All command line outputs including success and fail depends on match result, instead of providing extra arguments to writeln!()
    //Very confused since we are getting rid of println! and eprintln! to use writeln!, then what is the point of matching writeln!, which already takes in some error messages..?
    let _ = writeln!(&mut writer, "server needs exactly two arguments, which are program name and a network address", prog_name);
        // eprintln!("usage: {} <script_file_name>, [whinge]", prog_name);

    // println!("usage: {} <script_file_name>, [whinge]", prog_name);
}