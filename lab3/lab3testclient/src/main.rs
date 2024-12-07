//main.rs
//Author : DK Kim, donggyukim@wustl.edu
//main file that runs the "lab3testclient" program

use std::net::TcpStream;

const PROG_NAME : u8 = 0;
const NETWORK_ADDRESS : u8 = 1;
const ERROR_CODE : u8 = 1;
const TOKEN : u8 = 2;
const MIN_ARGS_FOR_TEST : u8 = 3;
const ONE_SECOND : u8 = 1;
const ZERO_SECOND : u8 = 1;

//Instruction 12.
fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != MIN_ARGS_FOR_TEST {
        usage(&args[PROG_NAME]);
        return ERROR_CODE;

    }

    let mut program_name = args[PROG_NAME].clone();
    let mut network_address = args[NETWORK_ADDRESS].clone();
    let mut token = args[TOKEN].clone();



    match TcpStream::connect(network_address.clone()) {
        Ok(stream) => {
            println!("Connected to the server at {}", network_address);

            stream.write_all(token.as_bytes())?;
            stream.flush()?;

            if token == "quit" {
                let one_second_duration = Duration::new(ONE_SECOND, ZERO_SECOND);
                std::thread::sleep(one_second_duration);
                TcpStream::connect(network_address.clone());
                return Ok(());
            }
            
        }
        Err(_) => {
            println!("Couldn't connect to server...");
        }
    }

}

fn usage(prog_name : &String) {
    let mut writer = std::io::stdout().lock();

    //All command line outputs including success and fail depends on match result, instead of providing extra arguments to writeln!()
    //Very confused since we are getting rid of println! and eprintln! to use writeln!, then what is the point of matching writeln!, which already takes in some error messages..?
    let _ = writeln!(&mut writer, "test client needs exactly three arguments, which are program name and a network address and a token");
        // eprintln!("usage: {} <script_file_name>, [whinge]", prog_name);

    // println!("usage: {} <script_file_name>, [whinge]", prog_name);
}