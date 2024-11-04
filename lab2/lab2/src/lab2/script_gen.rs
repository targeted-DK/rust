//script_gen.rs
//Author : DK Kim, donggyukim@wustl.edu
//file that contains grab_trimmed_file_lines function for this program
use super::declarations::*;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

//This function opens a txt file using BufReader and reads every line from the txt file 
//and pushes each line to vec_lines
pub fn grab_trimmed_file_lines(file_name: &String, vec_lines: &mut Vec<String>) -> Result<(), u8> {
    let file = match File::open(file_name) {
       
            Err(e) => {
                
                println!{"File {} cannot be opened : {} in grad_trimmed_file_lines()", file_name, e};

                return Err(FAIL_TO_SCRIPT);
            },

            Ok(file) => file,    
        };

            //Opens each file
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
       
