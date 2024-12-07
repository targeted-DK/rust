//play.rs
//Author : DK Kim, donggyukim@wustl.edu
//creates play struct and implements necessary functions.
use super::declarations::*;
use std::sync::atomic::Ordering;
use crate::lab3::scene_fragment::SceneFragment;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

const HAS_ELEMENTS: usize = 0;
const SCENE_INDEX : usize = 0;
const TITLE_NAME_INDEX : usize = 1; 
const CONFIG_FILE_NAME_INDEX : usize = 0;
const TOO_MANY_TOKENS : usize = 2;
const FISRT_SCENE : usize = 0;
const PREV_SCENE_OFFSET : usize = 1;
const NEXT_SCENE_OFFSET : usize = 1;
const NO_MORE_TOKENS : usize = 1;
const SIZE_OFFSET : usize = 1; 

type ScriptConfig = Vec<(bool, String)>;
//Instruction 3.
type Fragments = Vec<Arc<Mutex<SceneFragment>>>;

pub struct Play {
    fragments : Fragments,
}

impl Play {
    pub fn new() -> Self {
        Play {
            fragments : Fragments::new(),
        }
    }

    //This function fills in necessary information needed in Play's Fragment field using information stored in ScriptConfig struct.
    pub fn process_config(&mut self, script_config: &ScriptConfig) -> Result<(), u8> {

        
        let mut thread_handles = vec![];

        let mut title = String::new();
    
        
        for (is_scene_title, some_string) in script_config {

                if *is_scene_title {
                
                title = some_string.to_string();
                } else {

                    //Instruction 3.
                    let fragment = Arc::new(Mutex::new(SceneFragment::new(&title)));

                    self.fragments.push(fragment.clone());
                    
                    // let latest_fragment_index = self.fragments.len() - 1;

                    //Instruction 5 - Redoing this portion to use thread
                    // &self.fragments[latest_fragment_index];
                    let cloned_fragment = fragment.clone();
                    let cloned_string = some_string.clone();

                    // let mut output = todo!();
                    thread_handles.push(thread::spawn(move || {
                        let result = cloned_fragment.lock().unwrap().prepare(&cloned_string);
                        
                        result
                    }));

                    println!("{}", cloned_fragment.lock().unwrap().play_title);
                    // thread_handles.push( 
                    //     thread::spawn(move || {
                    //         let _ =  cloned_fragment.lock().unwrap().prepare(&cloned_string);
                    //         // cloned_fragment.lock().unwrap().prepare(&cloned_string);

                    //     }
                    //     )
                    // );
                    // self.fragments.push(output);
                

                
                    //This was a little tricky to me because once you push a fragment into self.fragments vector, 
                    //you cannot access fragment variable since it's moved.
                    //So I ended up accessing the last element in self.fragments vector 
                    //to call fragment.prepare()

                    // match self.fragments[latest_fragment_index].lock().unwrap().prepare(some_string) {

                    //     Ok(()) => {
                    //         // succeed, move on to the next tuple
                    //     },
                    //     Err(e) => {
                    //         let mut writer = std::io::stdout().lock();

                    //         let _ = writeln!(&mut writer,"Script generation failed in SceneFragment::prepare() in Play::process_confing() {}", e);
                    //         return Err(FAIL_TO_SCRIPT);
                    //     }
                    // }


                    // let latest_fragment_index = self.fragments.len() - 1;
                    // //due to the lifetime of self, I ended up cloning the fragment struct to 'move' into a thread
                    

                    // thread_handles.push( 
                    //     thread::spawn(move || self.fragments[latest_fragment_index].lock().unwrap().prepare(some_string))
                    //  );
                }

                
            
        }

        for handle in thread_handles {
            match handle.join() {
                Ok(_) => {} ,
                Err(_) => println!("{}", "Problem occured in this thread for process_config() in play.rs"),
            };
        }
        // self.fragments = thread_handles.into_iter().collect();
        

        Ok(())
        
    }

    //This function does sanity check for whether given line indicates a scene or a configuration file used in the scene.
    fn add_config(config_file_line : &String, script_config : &mut ScriptConfig) {
                          
        if config_file_line.trim().is_empty() {
            return;
        }

        let tokens : Vec<&str> = config_file_line.split_whitespace().collect();
       
        let mut string_container : String = String::new();
        string_container.push_str("");
      
        //If the a line in the config_file includes [scene] as the first token, it indicates that the line is the title of the fragment, which will be located in the head of the fragments vector
        if tokens[SCENE_INDEX] == "[scene]" {

            if tokens.len() == NO_MORE_TOKENS {

                if IS_WHINGE.load(Ordering::SeqCst) {
                    let mut writer = std::io::stdout().lock();

                    let _ = writeln!(&mut writer,"Warning : [scene] token exists but a scene title is missing in the file!");
                }

                return;

            } else {

                //If the a line in the config_file with a player name, it indicates that the line has the information about which file to read for the player's lines
                string_container = tokens[TITLE_NAME_INDEX..].join(" ");
                script_config.push((true, string_container));
            }
        } 
        else
        {

            string_container = tokens[CONFIG_FILE_NAME_INDEX].to_string();
            script_config.push((false, string_container));

            if tokens.len() > TOO_MANY_TOKENS && IS_WHINGE.load(Ordering::SeqCst) {
                let mut writer = std::io::stdout().lock();

                let _ = writeln!(&mut writer,"Warning : Too many tokens obtained while parsing the configuration file line in add_config: {:?} ", tokens);
            }
        }
    }

    //This function reads given configuration file to check which files we need to read to script the entire play
    fn read_config(config_file_name : &String, script_config : &mut ScriptConfig) -> Result<(), u8> {

        let file = match File::open(config_file_name) {

            Err(e) => {

                let mut writer = std::io::stdout().lock();

                let _ = writeln!(&mut writer,"File {} cannot be opened : {} in Play::read_config()", config_file_name, e);

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
                            //success 
                            break;
                        } else {

                            let trimmed_line =  line_holder.trim().to_string();

                            //Since we are adding line by line using add_config, instead of adding all trimmed lines to a vector like in lab1,
                            //add_config does sanity check of inputs into a ScriptConfig, not read_config().
                            Self::add_config(&trimmed_line, script_config);
                        }
                        
                    },
                    Err(e) => {
                        // let mut writer = std::io::stdout().lock();

                        // let result = writeln!(&mut writer,"");
                        //     match result {
                        //         Ok(v) => v,
                        //         Err(e) =>  e,
                        // }
                        let mut writer = std::io::stdout().lock();

                        let _ = writeln!(&mut writer,"Script generation has failed in read_config() : {}", e);
                        return Err(FAIL_TO_SCRIPT)
                    }
                }
            }
            
        Ok(())
    }


    //This prepare() in play struct works similar to prepare() in SceneFragment 
    //It reads/opens each configuration file
    //and creates SceneFragment and pushes them into Fragments vector field in Play struct.
    //Basically we are creating a blueprint of entire play by adding total number of scenes for a given play, starting here.
    pub fn prepare(&mut self, config_file_name : &String) -> Result<(), u8> {

        let mut script_config = ScriptConfig::new();
        

        match Self::read_config(config_file_name, &mut script_config) {
            Ok(()) => {
            // succeed, move on 
            },
            Err(e) => {

                // let mut writer = std::io::stdout().lock();

                // let result = writeln!(&mut writer,"");
                //     match result {
                //         Ok(v) => v,
                //         Err(e) => e,
                // }
                let mut writer = std::io::stdout().lock();

                let _ = writeln!(&mut writer,"Script generation failed in read_confing() in Play::prepare() : {}", e);
                return Err(FAIL_TO_SCRIPT);
            },
        };

        match self.process_config(&script_config){
            Ok(()) => {
            // succeed, move on 
            },
            Err(e) => {

                // let mut writer = std::io::stdout().lock();

                // let result = writeln!(&mut writer,"");
                //     match result {
                //         Ok(v) => v,
                //         Err(e) =>  e,
                // }
                let mut writer = std::io::stdout().lock();

                let _ = writeln!(&mut writer,"Script generation failed in process_config() in Play::prepare() : {}", e);
                return Err(FAIL_TO_SCRIPT);
            }, 
        };
      
        
        // if !(self.fragments.len() > HAS_ELEMENTS && !self.fragments[SCENE_INDEX].lock().unwrap().play_title.is_empty()) {
        //     let mut writer = std::io::stdout().lock();

        //     let _ = writeln!(&mut writer,"Script generation failed in prepare() to check fragments");
            
        //     return Err(FAIL_TO_SCRIPT);
        // } 

        Ok(())

    }

    pub fn recite(&mut self){


        for scene_num in FISRT_SCENE..self.fragments.len(){
            // println!("{}",  self.fragments[scene_num].lock().unwrap().players.len());

            //Enter the scene
            if scene_num == FISRT_SCENE {
                self.fragments[scene_num].lock().unwrap().enter_all();

                // unlocked_obj.enter_all()
                // self.fragments[scene_num].enter_all();
            } else {

                let unlocked_obj = self.fragments[scene_num].lock().unwrap();
                
                unlocked_obj.enter(&self.fragments[scene_num - PREV_SCENE_OFFSET].lock().unwrap())
                // self.fragments[scene_num].enter(&self.fragments[scene_num - PREV_SCENE_OFFSET])
            }
            //Script each scene fragment
            self.fragments[scene_num].lock().unwrap().recite();
            // println!("{}", self.fragments[scene_num].lock().unwrap().play_title);
            //Exit the scene
            if scene_num == self.fragments.len() - SIZE_OFFSET {
                let unlocked_obj = self.fragments[scene_num].lock().unwrap();

                unlocked_obj.exit_all();
                // self.fragments[scene_num].exit_all();
            } else {

                let unlocked_obj = self.fragments[scene_num].lock().unwrap();
                unlocked_obj.exit(&self.fragments[scene_num + NEXT_SCENE_OFFSET].lock().unwrap());
                // self.fragments[scene_num].exit(&self.fragments[scene_num + NEXT_SCENE_OFFSET]);
            }

        }

       
    }
}