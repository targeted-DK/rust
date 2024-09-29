//script_gen.rs
//Author : DK Kim, donggyukim@wustl.edu
//file that contains necessary functions for this program
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::sync::atomic::*;
use crate::declarations::*;

type PlayConfig = Vec<(String, String)>;
const TITLE_LINE_INDEX : usize = 0; 
const FIRST_CHARACTER_LINE_INDEX: usize = 1; 
const CHARACTER_NAME_INDEX: usize = 0; 
const FILE_NAME_INDEX: usize = 1;
const CONFIG_TOKEN_NUM: usize = 2;
const ZERO: usize = 0;

//I added this to indicate that read_line() reach the end of line, returning a value of 0
const EOL_VAL : usize = 0;

// This function does sanity check if we have proper line number and character line
fn add_script_line(play: &mut Play, unparsed_line: &String, character_name: &String) {

    if unparsed_line.len() > ZERO {

        if let Some((first_token, rest)) = unparsed_line.split_once(char::is_whitespace) {
         
            let trimmed_line = rest.trim();

            //reference : https://stackoverflow.com/questions/73849900/how-to-handle-string-parse-parseinterror-rust=
            match first_token.parse::<usize>() {

                Err(_) => {
                    // This error message is only printed when 'whinge' paramter is given as a 3rd parameter when you run the program. ex) './lab1 hamlet_ii_2_config.txt whinge'
                    if IS_PROBLEM.load(Ordering::SeqCst) {  

                        println!("This token does not represent a valid usize value : {}, line by {}", first_token, character_name);
                    
                    }
                },
                Ok(result) => {

                    play.push((result, character_name.to_string(), trimmed_line.to_string()));

                }
                
        }

        }
    }
}

//This function opens a txt file using BufReader and reads every line from the txt file 
//and pushes each line to vec_lines
fn grab_trimmed_file_lines(file_name: &String, vec_lines: &mut Vec<String>) -> Result<(), u8> {
    
    let file = match File::open(file_name) {
       
            Err(e) => {

                println!{"File {} cannot be opened : {}", file_name, e};

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
       
//This function uses grab_trimmed_lines() and add_script_line 
// to convert each lines from text file into appropriate format
fn process_config(play : &mut Play, play_config : &PlayConfig) -> Result<(), u8> {

        for (part_name, file_name) in play_config {
            
            let mut vec : Vec<String> = Vec::new();

            match grab_trimmed_file_lines(file_name, &mut vec) {
                Ok(()) => {
                   // succeed, move on 
                },
                Err(e) => {
                    println!("Script generation failed in process_config() : {}", e); 
                    return Err(FAIL_TO_SCRIPT);
                }
            }
           
            for line in vec {
                add_script_line(play, &line, part_name);
            }
        }
        Ok(())
}

//This function does sanity check whether txt files exist
fn add_config(config_file_line : &String, play_config : &mut PlayConfig) {
    
    let references : Vec<&str> = config_file_line.split_whitespace().collect();

    if references.len() != CONFIG_TOKEN_NUM {

        // This error message is only printed when 'whinge' paramter is given as a 3rd parameter when you run the program. ex) './lab1 hamlet_ii_2_config.txt whinge'
        if IS_PROBLEM.load(Ordering::SeqCst) {
            println!("Warning : Too few tokens or many tokens obtained : {}, Problem file : {}", references.len(), config_file_line)
        }

    } else if references.len() >= CONFIG_TOKEN_NUM {
        play_config.push((references[CHARACTER_NAME_INDEX].to_string(), references[FILE_NAME_INDEX].to_string()))
    }

}

//This function reads given configuration file to check which files we need to read to script the entire play
fn read_config(config_file_name : &String, play_title : &mut String, play_config : &mut PlayConfig) -> Result<(), u8> {
    let mut vec : Vec<String> = Vec::new();
   
    match grab_trimmed_file_lines(config_file_name, &mut vec) {
        Ok(()) => {
           // succeed, move on 
        },
        Err(e) => {
            println!("Script generation failed in read_config() : {}", e); 
            return Err(FAIL_TO_SCRIPT);
        }
    };

    if vec.len() < CONFIG_TOKEN_NUM {
        println!("Less than two tokens were obtained from reading lines in read_config : {} ", vec.len());
        return Err(FAIL_TO_SCRIPT); 
    };

    *play_title = vec[TITLE_LINE_INDEX].clone();

    //reference : https://stackoverflow.com/questions/38826633/how-to-skip-the-first-n-items-of-an-iterator-in-rust
    //vec.iter() returns reference object, so therefore we do not need to use &line
    for line in vec.iter().skip(FIRST_CHARACTER_LINE_INDEX) {
        add_config(line, play_config);
    };

    Ok(())
}

// This function is used main(), which is a starting point to script a play.
// calls read_config() to read configuration file to see which other text files to look at
// and then calls process_config() to process each text file into appropriate format we use in this program
pub fn script_gen(config_file_name : &String, play_title : &mut String, play : &mut Play) -> Result<(), u8> {
    let mut play_config = PlayConfig::new();
    
    match read_config(config_file_name, play_title, &mut play_config) {
        Ok(()) => {
          // succeed, move on 
        },
        Err(e) => {
            println!("Script generation failed in script_gen() : {}", e);
            return Err(FAIL_TO_SCRIPT);
        },
    };

    match process_config(play, &play_config){
        Ok(()) => {
          // succeed, move on 
         },
         Err(e) => {
             println!("Script generation failed in script_gen() : {}", e);
             return Err(FAIL_TO_SCRIPT);
         }, 
    };
    Ok(())
}


