//script_gen.rs
//Author : DK Kim, donggyukim@wustl.edu
//file that contains grab_trimmed_file_lines function for this program
use super::declarations::*;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
const INPUT_COMPONENT : u8 =  4;
const PREFIX_LEN : u8 = 4;
const ADDR_LEN : u8 = 13;
const PORT_LEN : u8 = 17;
const ERROR : u8 = 1;
const OFFSET : u8 = 1;
const IP_MAX : u8 =255;
const IP_MIN : u8 = 0;
const PORT_MIN : u8 = 0;
const PORT_MAX : u32 = 9999;
const ADDR_POS : u8 = 1;
const PORT_POS : u8 = 2;
const FILE_POS : u8 = 3;

//This function opens a txt file using BufReader and reads every line from the txt file 
//and pushes each line to vec_lines
pub fn grab_trimmed_file_lines(file_name: &String, vec_lines: &mut Vec<String>) -> Result<(), u8> {

    // println!("here");
    // println!("{}", file_name);
    // let result = match get_buffered_reader(file_name){
    //     Ok() => return Ok(),
    //     Err(e) => return Err(e), 
    // }
    let file = match File::open(file_name) {

            Err(e) => {
                
                println!{"File {} cannot be opened : {} in grad_trimmed_file_lines()", file_name, e};

                return Err(FAIL_TO_SCRIPT);
            },

            Ok(file) => file,    
        };

        
            let mut bf = BufReader::new(file);
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


// //Instruction 13.
// fn get_buffered_reader(input : &String) -> Result<BufReader<TcpStream>, u8> {

//     let prefix = &input[..PREFIX_LEN];

//     //when the input string is  in format, sanitize the input string whether it contains 
//     //correct contents
//     if prefix != "net:" {


//     //"net:127.0.0.1:7777:partial_macbeth_act_i_script.txt")
//     let input_parts : Vec<&str> = input.split(':').collect();

//      //sanitize ip address
//     if input_parts.len() != INPUT_COMPONENT {
//         return Err(Err, "Invalid ip address format");
//     }

//     let addr_parts = input_parts[ADDR_POS].clone();
//     let addr_parts_splited = addr_parts.split('.');

//     let port_part = input_parts[PORT_POS].clone();

//     for addr_part in addr_parts_splited {
//         //reference: https://www.reddit.com/r/learnrust/comments/14x0mol/how_do_i_parse_a_string_to_find_numeric_data/
//         let parsed_addr: u32 = match addr_part.trim().parse() {
//             Ok(num) => num, 
//             Err(_) => {
//                 println!("make sure you added ip address in the correct format!");
//                 return Err();
//             }
//         };

//         if parsed_addr > IP_MAX || parsed_addr < IP_MIN {
//             println!("IP address component out of range: {}", parsed_addr);
//             return Err();
//         }
//     }

//     //sanitize port num
//     let parsed_port_num : u32 = match port_part.trim().parse() {
//         Ok(num) => num, 
//         Err(_) => {
//             println!("make sure you added port number in the correct format!");
//             return Err();
//         }
//     }
//     if parsed_port_num > PORT_MAX || parsed_port_num < PORT_MIN {
//         println!("port number component out of range: {}", parsed_port_num);
//         return Err();
//     }

//     let full_addr = addr_parts + ":" + port_part;

//     let mut stream = match TcpStream::connect(full_addr) {
//         Ok(stream) => stream,
//         Err(x) => {
//             return BufReader::new(input);
//         }

//     };

//     let file_part = input_parts[FILE_POS].clone();

//     let mut file = match File::open(file_part){
//         Ok(f) => f,
//         Err(e) => {
//             return BufReader::new(input);
//         }
//     }

//     stream.write_all(file.as_bytes())?;
//     return Ok(BufReader::new(stream))
//     }

//     //when the input string is not in format
//     let mut file = File::open(input)?;
//     return Ok(BufReader::new(file));


// }
