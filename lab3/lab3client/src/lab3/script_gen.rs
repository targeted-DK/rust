//script_gen.rs
//Author : DK Kim, donggyukim@wustl.edu
//file that contains grab_trimmed_file_lines / get_buffered_reader functions, which sanitizes inital inputs provided by command lines for this program
use super::declarations::*;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::net::TcpStream;
use std::io::Write;
use std::io::Read;

const INPUT_COMPONENT : usize =  4;
const PREFIX_LEN : usize = 4;
const ERROR : u8 = 1;
const IP_MAX : u32 =255;
const IP_MIN : u32 = 0;
const PORT_MIN : u32 = 0;
const PORT_MAX : u32 = 9999;
const ADDR_POS : usize = 1;
const PORT_POS : usize = 2;
const FILE_POS : usize = 3;

//This function opens a txt file using BufReader and reads every line from the txt file 
//and pushes each line to vec_lines
pub fn grab_trimmed_file_lines(file_name: &String, vec_lines: &mut Vec<String>) -> Result<(), u8> {

    
    //check whether input string is a filename or a string with ip address, port, and a filename using get_buffered_reader function.
    let mut bf = match get_buffered_reader(file_name) {
        Ok(reader) => reader, 
        Err(e) => {
            println!("File {} cannot be opened or accessed: {} in grab_trimmed_file_lines()", file_name, e);
            return Err(FAIL_TO_SCRIPT);
        }
    };
        
    let mut line_holder : String = String::new();

    loop {  

        //then reads every line in the text file until End of Line is reached
        //and pushes each line to vec_lines.
        line_holder.clear();

        match bf.read_line(&mut line_holder) {
            
            Ok(val) => {

                if val == EOL_VAL {
                    return Ok(());
                } else {
                    let trimmed_line =  line_holder.trim();
                    vec_lines.push(trimmed_line.to_string());
                }
                
            },
            Err(e) => {
                println!("Script generation has failed in grab_trimmed_file_lines() : {}", e);
                return Err(FAIL_TO_SCRIPT)
            }
        }
    }
}


fn get_buffered_reader(input : &String) -> Result<BufReader<Box<dyn Read>>,u8> {

    let prefix = &input[..PREFIX_LEN];

    //when the input string starts with "net:" sanitize the input string
    if prefix == "net:" {

    //example input : "net:127.0.0.1:7777:partial_macbeth_act_i_script.txt"
    let input_parts : Vec<&str> = input.split(':').collect();

     //sanitize input string components
    if input_parts.len() != INPUT_COMPONENT {
        return Err(ERROR);
    };

    let addr_parts = input_parts[ADDR_POS].clone();
    let addr_parts_splited = addr_parts.split('.');
    let port_part = input_parts[PORT_POS].clone();

    //sanitize ip adress
    for addr_part in addr_parts_splited {
        //reference: https://www.reddit.com/r/learnrust/comments/14x0mol/how_do_i_parse_a_string_to_find_numeric_data/
        let parsed_addr: u32 = match addr_part.trim().parse() {

            Ok(num) => num, 

            Err(_) => {
                println!("make sure you added ip address in the correct format!");
                return Err(ERROR);
            }
        };

        if parsed_addr > IP_MAX || parsed_addr < IP_MIN {
            println!("IP address component out of range: {}", parsed_addr);
            return Err(ERROR);
        }
    };

    //sanitize port num
    let parsed_port_num : u32 = match port_part.trim().parse() {
        
        Ok(num) => num, 
        Err(_) => {
            println!("make sure you added port number in the correct format!");
            return Err(ERROR);
        }

    };

    if parsed_port_num > PORT_MAX || parsed_port_num < PORT_MIN {

        println!("port number component out of range: {}", parsed_port_num);

        return Err(ERROR);
    };

    let full_addr = addr_parts.to_owned() + ":" + port_part;

    let mut stream = match TcpStream::connect(full_addr) {

        Ok(stream) => stream,

        Err(_) => {

            return Err(ERROR);
        }

    };

    let file_part = input_parts[FILE_POS].clone();


    let _ = match stream.write_all(file_part.as_bytes()){

            Ok(_) => {
                //continue;
            }
            Err(_) => {

                return Err(ERROR);
            }
        };

        return Ok(BufReader::new(Box::new(stream)))
    }

    //input as a string, not with ip addr and a port
    //when the input string is not in format
    let file = match File::open(input){
        Ok(f) => f,
        Err(_) => {
            return Err(ERROR);
        }
    };

    //reference: https://users.rust-lang.org/t/how-to-return-bufreader/34651
    let bf = BufReader::new(file);

    return  Ok(BufReader::new(Box::new(bf)))

}
