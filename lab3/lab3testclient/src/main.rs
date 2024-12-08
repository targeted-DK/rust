//main.rs
//Author : DK Kim, donggyukim@wustl.edu
//main file that runs the "lab3testclient" program to check is lab3server runs properly
use std::net::TcpStream;
use std::io::Write;
use std::env;
use std::time::Duration;
use std::io::BufReader;
use std::io::BufRead;

const NETWORK_ADDRESS : usize = 1;
const ERROR_CODE : u8 = 1;
const TOKEN : usize = 2;
const MIN_ARGS_FOR_TEST : usize = 3;
const ONE_SECOND : u64 = 1;
const ZERO_SECOND : u32 = 1;

fn main() -> Result<(), u8> {

    //check number of inputs
    let args: Vec<String> = env::args().collect();
    if args.len() != MIN_ARGS_FOR_TEST {
        usage();
        return Err(ERROR_CODE);

    }

    let network_address = args[NETWORK_ADDRESS].clone();
    let token = args[TOKEN].clone();

    println!("Connecting to: {}", network_address);
    println!("Token: {}", token);


    let mut stream = match TcpStream::connect(&network_address) {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Failed to connect to server: {}", e);
            return Err(ERROR_CODE);
        }
    };


            if let Err(e) = stream.write_all(token.as_bytes()) {
                eprintln!("Token failed to send over server: {}", e);
                return Err(ERROR_CODE);
            }

        


    if token == "quit" {
        let one_second_duration = Duration::new(ONE_SECOND, ZERO_SECOND);
        std::thread::sleep(one_second_duration);
        let _ = TcpStream::connect(network_address.clone());

        return Ok(()); 
    } else {


        let reader = BufReader::new(stream.try_clone().unwrap());

        for line in reader.lines() {
            match line {
                Ok(line) => {
                    println!("Line : {}", line);
                }
                Err(e) => {
                    println!("failed to read line : {}", e);
                    return Err(ERROR_CODE);
                }
            }
        }
                                
        
        }
        

    Ok(())
}

fn usage() { 
    let mut writer = std::io::stdout().lock();
    let _ = writeln!(&mut writer, "This test client needs exactly three arguments, which are program name and a network address and a token");
    let _ = writeln!(&mut writer, "usage: ./lab3testclient <network address>, <token>");
}