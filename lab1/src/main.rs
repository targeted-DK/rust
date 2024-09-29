//main.rs
//Author : DK Kim, donggyukim@wustl.edu
//main file that runs the program
use std::env;
use std::sync::atomic::*;
mod declarations;
mod script_gen;
use crate::declarations::*;
use crate::script_gen::script_gen;

fn main() -> Result<(), u8> {

    //https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
    let args: Vec<String> = env::args().collect();

    if args.len() < MIN_ARGS {
        usage(&args[declarations::PROG_NAME]);
        return Err(declarations::BAD_ARGS);
    }

    let mut config_arg = args[declarations::CONFIG_NAME].clone();

    match parse_args(&mut config_arg) {
        Ok(()) => {
            // succeed, move on 
        },
        Err(_) => { return Err(declarations::BAD_ARGS) }
    }

    let mut play_title = String::new();
    let mut play = declarations::Play::new();

    //This is the only function from script_gen file used in main() to start scripting a play based on given files.
    match script_gen(&config_arg, &mut play_title, &mut play) {
        Ok(()) => {
            // succeed, move on 
        },
        Err(e) => {
            println!("Script generation has failed in script_gen() in main : {}", e);
            return Err(declarations::FAIL_TO_SCRIPT)
        }
    };
  
    play.sort();

    recite(&play_title, &play);

    Ok(())
}

fn usage(prog_name : &String) {
    println!("usage: {} <configuration_name_file>, [whinge]", prog_name);
}

fn parse_args(config_arg : &mut String) -> Result<(), u8>{

    let mut vec = Vec::new();

    for arg in env::args(){

        vec.push(arg.to_string());

    }

    // checks if we have proper number of arguments when running the program.
    if vec.len() < MIN_ARGS || vec.len() > MAX_ARGS {

        usage(&vec[declarations::PROG_NAME]);

        return Err(BAD_ARGS);
    }

    if vec.len() == MAX_ARGS && vec[OPT_ARG] != "whinge" {

        //prints usage and return an error if the third argument is invalid
        usage(&vec[declarations::PROG_NAME]);

        return Err(BAD_ARGS);
    }

        *config_arg  = vec[declarations::CONFIG_NAME].clone();

        //This if statement checks whether 'whinge' is passed as a third argument
        //IS_PROBLEM variable, if set to true, prints out warning message when we have problems parsing congfiguration files
        //or each lines in text files.
        if vec.len() == MAX_ARGS && vec[OPT_ARG] == "whinge" {

            declarations::IS_PROBLEM.store(true, Ordering::SeqCst);

        }

        Ok(())
}

fn recite(title : &String, play : &declarations::Play){

    println!("Title of this play is {}", title);    

    let mut current_character = String::new();
    
    //This for loop iteratively prints out line number and character text
    //Prints out character name once until a different character's turn comes.
    for (line_num, character, text) in play {

            if *character != current_character {

                println!();

                println!("{}.", character);
                //reference : Conversation with Eric during studio on 9/5 - I was stuck on this issue : expected `String`, found `&mut String`
                current_character = character.clone();
            }
            
        println!("{}: {}", line_num, text);
        
    }
    

    

    
}