//player.rs
//Author : DK Kim, donggyukim@wustl.edu
//creates player struct and implements necessary functions.
use super::declarations::*;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::sync::atomic::Ordering;
use std::cmp::Ordering as CmpOrdering;


const INDEX_INIT : usize = 0;
const EMPTY : usize = 0;
const FIRST_LINE : usize = 0;
const NEXT_LINE_BY_ONE : usize = 1;
const PANIC_IN_PROCESS : usize = 3;

pub type PlayLines = Vec<(usize, String)>;

pub struct Player {
   pub character_name : String,
   pub character_lines : PlayLines,
   pub entry_index : usize,
}

impl Player {
    pub fn new(name : String) -> Self {
        Player {
            character_name : name,
            character_lines : PlayLines::new(),
            entry_index : INDEX_INIT,
        }
    }


    // This function does sanity check if we have proper line number and character line
    fn add_script_line(&mut self, unparsed_line : &String) {

        if unparsed_line.len() > EMPTY {

            if let Some((index, text)) = unparsed_line.split_once(char::is_whitespace) {
            
                let trimmed_line = text.trim();

                //reference : https://stackoverflow.com/questions/73849900/how-to-handle-string-parse-parseinterror-rust=
                match index.parse::<usize>() {

                    Err(_) => {
                        // This error message is only printed when 'whinge' paramter is given as a 3rd parameter when you run the program. ex) './lab1 hamlet_ii_2_config.txt whinge'
                        if IS_WHINGE.load(Ordering::SeqCst) {  

                            let mut writer = std::io::stdout().lock();

                            let _ = writeln!(&mut writer,"This token does not represent a valid usize value : {}, line by {}", index, self.character_name);
                        
                        }
                    },
                    Ok(line_num) => {

                        self.character_lines.push((line_num, trimmed_line.to_string()))
                    }
                    
            }

            }
        }
    }

    //this function 'prepares' each player's line by opening a correct file and add each line to a player struct's PlayLines field 
    pub fn prepare(&mut self, file_name : &String) -> Result<(), u8> {

            // println!("{}", file_name);
            let file = match File::open(file_name) {
       
            Err(e) => {

                let mut writer = std::io::stdout().lock();

                //Instruction 5-3
              
                let _ = writeln!(&mut writer,"File {} cannot be opened : {}", file_name, e);
                
                std::panic::panic_any(PANIC_IN_PROCESS);
                // return Err(FAIL_TO_SCRIPT);
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
                            //success
                            break;
                        } else {
                            let trimmed_line =  line_holder.trim();
                           
                            self.add_script_line(&trimmed_line.to_string());
                        }
                        
                    },
                    Err(e) => {
                        let mut writer = std::io::stdout().lock();

                        let _ = writeln!(&mut writer,"Script generation has failed in grab_trimmed_file_lines() : {}", e);

                        std::panic::panic_any(PANIC_IN_PROCESS);

                        // return Err(FAIL_TO_SCRIPT)
                    }
                }
            }

            // println!("{}", self.character_lines.len());
            Ok(())

    }

    //A player speaks each lineprintln!
    //This is a important function that prints out correct line in the scene by
    //comparing a player's current index (self.entry_index) with total number of lines that the player needs to speak in a scene/
    pub fn speak(&mut self, recent_speaker : &mut String) {
        
        if  self.entry_index < self.character_lines.len()  {

            if *recent_speaker != self.character_name  {

                *recent_speaker = self.character_name.clone();
                println!("");
                println!("{}", recent_speaker);

            } 

            let (line_number, line_text) = &self.character_lines[self.entry_index];

            println!("{} {}", line_number, line_text);

            self.entry_index += NEXT_LINE_BY_ONE;
        } 

    }

    //This function checks if next line for a player exists. This function is used in recite() 
    // to determine if player is done talking in a scene or not.
    pub fn next_line(&self) -> Option<usize> {
     
        if self.entry_index < self.character_lines.len() {

            Some(self.character_lines[self.entry_index].0)

        } else {

            None
            
        }
    }
}

//reference : https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {

        Some(self.cmp(other))
    }
}

//reference : https://stackoverflow.com/questions/47852269/can-i-use-and-in-match
impl Ord for Player {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {

        match (self.character_lines.len() == EMPTY, other.character_lines.len() == EMPTY) {
            (true, false) => CmpOrdering::Less,
            (false, true) => CmpOrdering::Greater,
            (true, true) => CmpOrdering::Equal,
            (false, false) => {
                self.character_lines[FIRST_LINE].0.cmp(&other.character_lines[FIRST_LINE].0)
            }
        }
    }    
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {

        (self.character_lines.len() == EMPTY && other.character_lines.len() == EMPTY) 
        || (self.character_lines[FIRST_LINE].0 == other.character_lines[FIRST_LINE].0)

    }

}

impl Eq for Player {}


