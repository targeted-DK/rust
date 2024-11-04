//scene_fragment.rs
//Author : DK Kim, donggyukim@wustl.edu
//creates SceneFragment struct and implements necessary functions
use super::declarations::*;
use super::player::*;
use std::sync::atomic::*;
use crate::lab2::script_gen::grab_trimmed_file_lines;

const INIT_LINE_NUM : usize = 0;
const START_COUNTER: usize = 0;
const CHARACTER_NAME_INDEX: usize = 0; 
const NEXT_LINE_BY_ONE : usize = 1;
const ONE_PLAYER_DONE : usize = 1;
const FILE_NAME_INDEX: usize = 1;
const CONFIG_TOKEN_NUM: usize = 2;

type PlayConfig = Vec<(String, String)>;

pub struct SceneFragment {
    pub play_title : String,
    pub players : Vec<Player>,
}

impl SceneFragment {
    pub fn new(title : &String) -> Self {
        SceneFragment {
            play_title : title.clone(),
            players : Vec::new(),
        }
    }

    //This function uses information we got from each configurations line into actual player struct 
    //calls player.prepare() to open file designated for each player of a scene
    pub fn process_config(&mut self, play_config: &PlayConfig) -> Result<(), u8> {
        for (part_name, file_name) in play_config {
         
           
            let mut player = Player::new(part_name.to_string());
        
            match player.prepare(file_name) {
                Ok(()) => {
                    // succeed, move on 
                    self.players.push(player);
                },
                Err(e) => {
                    println!("Failed to prepare player {} , error :{} in SceneFragment::process_config()", file_name, e);
                    return Err(FAIL_TO_SCRIPT);
                }
            }
        }

       
        Ok(())
        
    }

    //This function checks if correct number of tokens (2) (player name, file name) exists in each configuration file line
    fn add_config(config_file_line : &String, play_config : &mut PlayConfig) {
             
     
        let references : Vec<&str> = config_file_line.split_whitespace().collect();
       
        if references.len() != CONFIG_TOKEN_NUM {

            if IS_WHINGE.load(Ordering::SeqCst) {
                eprintln!("Warning : Too few tokens or many tokens obtained : {}, Problem file : {}", references.len(), config_file_line)
            }

        } else if references.len() >= CONFIG_TOKEN_NUM {

            play_config.push((references[CHARACTER_NAME_INDEX].to_string(), references[FILE_NAME_INDEX].to_string()))
        }

       
    }

    //This function reads/opens given configuration files 
    fn read_config(config_file_name : &String, play_config : &mut PlayConfig) -> Result<(), u8> {
      
        let mut config_lines : Vec<String> = Vec::new();
      
        let _ =  grab_trimmed_file_lines(config_file_name, &mut config_lines);

        //reference : https://stackoverflow.com/questions/38826633/how-to-skip-the-first-n-items-of-an-iterator-in-rust
        for config_file_line in config_lines.iter(){
            
            Self::add_config(config_file_line, play_config);
        };

        Ok(())
    }


    // This function calls read_config() and process_config(), to read scene name/configuration file name needed to script each scene
    pub fn prepare(&mut self, config_file_name : &String) -> Result<(), u8> {
        let mut play_config = PlayConfig::new();

        match Self::read_config(config_file_name, &mut play_config) {
            Ok(()) => {
                
            // succeed, move on 
            },
            Err(e) => {
                println!("Script generation failed in read_config() in SceneFragment::prepare() : {}", e);
                return Err(FAIL_TO_SCRIPT);
            },
        };        
      

        match self.process_config(&play_config){
            Ok(()) => {
            // succeed, move on 
            },
            Err(e) => {
                println!("Script generation failed in process_config() in SceneFragment::prepare() : {}", e);
                return Err(FAIL_TO_SCRIPT);
            }, 
        };

        self.players.sort();

        Ok(())
    }

    pub fn recite(&mut self) {

    let mut current_character = String::new();
    let mut current_line_num = INIT_LINE_NUM;
    let mut done_player_num = START_COUNTER;
    let mut player_done = vec![false; self.players.len()]; 

        //we will keep running this loop until no players has line to speak
        while done_player_num < self.players.len() {

        // This for loop checks if player is done talking in this script.
        // https://stackoverflow.com/questions/28991050/how-to-iterate-a-vect-with-the-indexed-position
        // players had to be iterated with iter_mut() because due to this error : `player` is a `&` reference, so the data it refers to cannot be borrowed as mutable
        for (i, player) in self.players.iter_mut().enumerate() {
            if player_done[i] {
                continue; 
            }

            //for every line, we will iterate through every player to see who gets to speak for the given line number
            if let Some(next_line_num) = player.next_line() {

                //Check if the first line exists
                if current_line_num == INIT_LINE_NUM && next_line_num > INIT_LINE_NUM {
                    if IS_WHINGE.load(Ordering::SeqCst) {
                        eprintln!("Line number 0 is missing in player {}'s script", player.character_name);
                    }
                }

                //Check if next line exists but also missing line numbers exist
                if next_line_num > current_line_num && next_line_num > current_line_num + NEXT_LINE_BY_ONE {
                    if IS_WHINGE.load(Ordering::SeqCst) {
                        eprintln!("Line number {} is missing in player {}'s script", current_line_num, player.character_name);
                    }
                }

                //Speak since next line exists (duplicate lines are spoken but missing lines are skipped)
                if current_line_num == next_line_num {
                    player.speak(&mut current_character);
                }

            } else {

                //Mark that this player is done talking because player.next_line is None and increment number of done player by 1.
                if !player_done[i] {
                    player_done[i] = true;
                    done_player_num += ONE_PLAYER_DONE;

                    if IS_WHINGE.load(Ordering::SeqCst) {
                        eprintln!("The line number is missing in player {}'s script after line number {}", player.character_name, current_line_num);
                    }
                }
            }
        }
            //Increment the current index line by one to keep track of overall index used to script the current scene.
            current_line_num += NEXT_LINE_BY_ONE;
    }
}


    //All players that did not exist in previous scene enters 
    pub fn enter(&self, other : &Self) {

        let mut names : Vec<String> = Vec::new();

        for player in &other.players {
            names.push(player.character_name.clone());
        }

        for player in &self.players {
            if !names.contains(&player.character_name) {
                println!("[Enter {}.]", player.character_name.clone());
            }
        }

    }

    //All players for the first scene enters
    pub fn enter_all(&self){
        println!("{}","");
        if !self.play_title.trim().is_empty(){
            println!("{}", self.play_title);
        }
        println!("{}","");

        for player in &self.players {
            println!("[Enter {}.]", player.character_name);
        }
    }

    //Players that will not show up in the next scene leaves
    pub fn exit(&self, other : &Self){
        if !self.play_title.trim().is_empty(){
            println!("Title : {}", self.play_title);
        }

        let mut names : Vec<String> = Vec::new();

        for player in &other.players {
            names.push(player.character_name.clone());
        }

        for player in self.players.iter().rev() {
            if !names.contains(&player.character_name) {
                println!("[Exit {}.]", player.character_name);
            }
        }
    }

     //All players exists. All Scenes are done.
    pub fn exit_all(&self) {
        for player in self.players.iter().rev() {
            println!("[Exit {}.]", player.character_name);
        }
       println!("{}","");
    }
}