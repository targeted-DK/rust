//scene_fragment.rs
//Author : DK Kim, donggyukim@wustl.edu
//creates SceneFragment struct and implements necessary functions
use super::declarations::*;
use super::player::*;
use std::sync::atomic::*;
use crate::lab3::script_gen::grab_trimmed_file_lines;
use std::io::Write;
use std::sync::Mutex;
use std::sync::Arc;
use std::thread;
// use std::cmp::Ordering::*;

const INIT_LINE_NUM : usize = 0;
const START_COUNTER: usize = 0;
const CHARACTER_NAME_INDEX: usize = 0; 
const NEXT_LINE_BY_ONE : usize = 1;
const ONE_PLAYER_DONE : usize = 1;
const FILE_NAME_INDEX: usize = 1;
const CONFIG_TOKEN_NUM: usize = 2;
const PANIC_IN_PROCESS : usize = 4;

type PlayConfig = Vec<(String, String)>;

pub struct SceneFragment {
    pub play_title : String,
    pub players : Vec<Arc<Mutex<Player>>>,
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


        let mut thread_handles = vec![];


        for (part_name, file_name) in play_config {
            // println!("{}",part_name);
            // println!("{}",file_name);

            let player = Arc::new(Mutex::new(Player::new(part_name.to_string())));

          
            let cloned_player = player.clone();
            let cloned_file_name = file_name.clone();
     

            thread_handles.push( 
                thread::spawn(move || {
                    let result = cloned_player.lock().unwrap().prepare(&cloned_file_name);
                    // cloned_fragment.lock().unwrap().prepare(&cloned_string);
                    result
                }
                )
            );
            
            // match player.lock().unwrap().prepare(file_name) {
            //     Ok(()) => {
                    // succeed, move on 
                    //I tried not to use clone() but rather use original player variable declared above, but I could not figure out how to bypass this error :
//         let mut player = Arc::new(Mutex::new(Player::new(part_name.to_string())));
//    |                 ---------- binding `player` declared here
//    ...
//    44 |             match &player.lock().unwrap().prepare(file_name) {
//       |                    ----------------------
//       |                    |
//       |                    borrow of `player` occurs here
//       |                    a temporary with access to the borrow is created here ...
//    ...
//    47 |                     self.players.push(player);
//       |                                       ^^^^^^ move out of `player` occurs here
//    ...
//    56 |             };
//       |              - ... and the borrow might be used here, when that temporary is dropped and runs the `Drop` code for type `MutexGuard`
                

            //     //     self.players.push(player.clone());
            //     // },
            //     Err(e) => {
            //         let mut writer = std::io::stdout().lock();

            //         let _ = writeln!(&mut writer,"Failed to prepare player {} , error :{} in SceneFragment::process_config()", file_name, e);
            //         // println!("Failed to prepare player {} , error :{} in SceneFragment::process_config()", file_name, e);
            //         return Err(FAIL_TO_SCRIPT);
            //     }
            // };

            
        }

        for handle in thread_handles {

            // handle.join().unwrap();
            match handle.join() {
                Ok(result) => {} ,
                Err(_) => println!("{}", "Problem occured in this thread for process_config() in scene_fragment.rs"),
            };
        }


        Ok(())
        
    }

    //This function checks if correct number of tokens (2) (player name, file name) exists in each configuration file line
    fn add_config(config_file_line : &String, play_config : &mut PlayConfig) {
      
        let references : Vec<&str> = config_file_line.split_whitespace().collect();
        if references.len() != CONFIG_TOKEN_NUM {

            if IS_WHINGE.load(Ordering::SeqCst) {
                
                let mut writer = std::io::stdout().lock();

                let _ = writeln!(&mut writer, "Warning : Too few tokens or many tokens obtained : {}, Problem file : {}", references.len(), config_file_line);
            }

        } else if references.len() >= CONFIG_TOKEN_NUM {

            play_config.push((references[CHARACTER_NAME_INDEX].to_string(), references[FILE_NAME_INDEX].to_string()))
        }


    }

    //This function reads/opens given configuration files 
    fn read_config(config_file_name : &String, play_config : &mut PlayConfig) -> Result<(), u8> {

        let mut config_lines : Vec<String> = Vec::new();
        // println!("here in readconfig");
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
            Err(_) => {
                // let mut writer = std::io::stdout().lock();

                // let _ = writeln!(&mut writer,"Script generation failed in read_config() in SceneFragment::prepare() : {}", e);

                std::panic::panic_any(PANIC_IN_PROCESS);

                // return Err(FAIL_TO_SCRIPT);
            },
        };        


        match self.process_config(&play_config){
            Ok(()) => {
            // succeed, move on 
            },
            Err(_) => {

                std::panic::panic_any(PANIC_IN_PROCESS);
                // let mut writer = std::io::stdout().lock();
                // let _ = writeln!(&mut writer,"Script generation failed in process_config() in SceneFragment::prepare() : {}", e);
                // return Err(FAIL_TO_SCRIPT);
            }, 
        };

        
    
        //Instruction 4-3 
        self.players.sort_by(|a, b| compare(a,b));


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
          

            //For every line, we will iterate through every player to see who gets to speak for the given line number
            //Added in Lab3 : Unlike lab2, we will first acquire a lock for each player with mut before we do anything with it.
            if let Ok(mut player_guard) = player.lock() {
                if let Some(next_line_num) = player_guard.next_line() {

               
                //Check if the first line exists
                if current_line_num == INIT_LINE_NUM && next_line_num > INIT_LINE_NUM {
                    if IS_WHINGE.load(Ordering::SeqCst) {
                        let mut writer = std::io::stdout().lock();

                        let _ = writeln!(&mut writer,"Line number 0 is missing in player {}'s script", player_guard.character_name);
                    }
                }
                
                //Check if next line exists but also missing line numbers exist
                if next_line_num > current_line_num && next_line_num > current_line_num + NEXT_LINE_BY_ONE {
                    if IS_WHINGE.load(Ordering::SeqCst) {
                        let mut writer = std::io::stdout().lock();

                        let _ = writeln!(&mut writer,"Line number {} is missing in player {}'s script", current_line_num, player_guard.character_name);
                    }
                }

               
              
                
                if current_line_num == next_line_num {
                 
                    player_guard.speak(&mut current_character);
                   
                }
               

            } else {

                //Mark that this player is done talking because player.next_line is None and increment number of done player by 1.
                if !player_done[i] {
                    player_done[i] = true;
                    done_player_num += ONE_PLAYER_DONE;

                    if IS_WHINGE.load(Ordering::SeqCst) {
                        let mut writer = std::io::stdout().lock();

                        let _ = writeln!(&mut writer,"The line number is missing in player {}'s script after line number {}", player_guard.character_name, current_line_num);
                    }
                }
            }
        }
            //Increment the current index line by one to keep track of overall index used to script the current scene.
        }
            current_line_num += NEXT_LINE_BY_ONE;
        
    }
}


    //All players that did not exist in previous scene enters 
    //Added in Lab3 : Unlike lab2, we will first acquire a lock for each player with mut before we do anything with it.
    pub fn enter(&self, other : &Self) {

        let mut names : Vec<String> = Vec::new();

        for player in &other.players {
            names.push(player.lock().unwrap().character_name.clone());
        }

        for player in &self.players {
            if !names.contains(&player.lock().unwrap().character_name) {
                println!("[Enter {}.]", player.lock().unwrap().character_name.clone());
            }
        }

    }

    //All players for the first scene enters
    pub fn enter_all(&self){
   

        // println!("{}",self.play_title); 
        // for player in &self.players {
        //     println!("{}",player.lock().unwrap().character_name);
        // }
        
        println!("{}","");
        if !self.play_title.trim().is_empty(){
            println!("{}", self.play_title);

        }
        println!("{}","");    

        for player in &self.players {
            println!("[Enter {}.]", player.lock().unwrap().character_name);
        }
    }

    //Players that will not show up in the next scene leaves
    pub fn exit(&self, other : &Self){
        if !self.play_title.trim().is_empty(){
            println!("Title : {}", self.play_title);
        }

        let mut names : Vec<String> = Vec::new();

        for player in &other.players {
            names.push(player.lock().unwrap().character_name.clone());
        }

        for player in self.players.iter().rev() {
            if !names.contains(&player.lock().unwrap().character_name) {
                println!("[Exit {}.]", player.lock().unwrap().character_name);
            }
        }
    }

     //All players exists. All Scenes are done.
    pub fn exit_all(&self) {
        for player in self.players.iter().rev() {
            println!("[Exit {}.]", player.lock().unwrap().character_name);
        }
            println!("{}","");
    }
}


//Instruction 4-2
fn compare(first_struct : &Arc<Mutex<Player>>, second_struct : &Arc<Mutex<Player>>) -> std::cmp::Ordering {
    
    let first_lock_result = match first_struct.lock() {
        Ok(v) => {
            v
        },
        Err(_) => {
            let mut writer = std::io::stdout().lock();
            let _ = writeln!(&mut writer, "There is a problem obtaining lock from the first_struct in scene_fragment::compare()");

            return std::cmp::Ordering::Equal;
        }
    };

    let second_lock_result = match second_struct.lock() {
        Ok(v) => {
            v
        },
        Err(_) => {
            let mut writer = std::io::stdout().lock();
            let _ = writeln!(&mut writer, "There is a problem obtaining lock from the second_struct in scene_fragment::compare()");

            return std::cmp::Ordering::Equal;
        }
    };

    let first_struct_content = match first_lock_result{
        ref v => v,
        };

    let second_struct_content = match second_lock_result {
        ref v => v,
    };


    match Player::partial_cmp(&first_struct_content, &second_struct_content) {
        Some(value) => { return value; },
        None => return std::cmp::Ordering::Equal,

    };
   
}
