//main.rs
//Author : DK Kim, donggyukim@wustl.edu
//main file that runs the "lab3client" program
use std::env;
use std::sync::atomic::*;

pub mod lab3;
use lab3::play::Play;
use crate::lab3::return_wrapper::*;
use lab3::declarations::*;
use std::io::Write;

const MIN_ARGS : usize = 2;
const MAX_ARGS : usize = 3;
const PROG_NAME : usize = 0;
const CONFIG_NAME : usize = 1;
const OPT_ARG : usize = 2;
const BAD_ARGS : u8 = 1;


fn main() -> ReturnWrapper {

    //https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
    let args: Vec<String> = env::args().collect();

    if args.len() < MIN_ARGS {
        usage(&args[PROG_NAME]);
        return ReturnWrapper::new(BAD_ARGS.into());
    }

    let mut config_arg = args[CONFIG_NAME].clone();

    match parse_args(&mut config_arg) {
        Ok(()) => {
            // succeed, move on 
        },
        Err(error) => { return ReturnWrapper::new(error.into()) }
    }

    let mut play = Play::new();

    match play.prepare(&config_arg) {
        Ok(()) => {

         
            play.recite();
            
        },
        Err(error) => {
            let mut writer = std::io::stdout().lock();

            let _ =  writeln!(&mut writer, "Script generation failed in play.prepare() called in main()");
            // if let Err(e) = writeln!(&mut writer, "main()::play.prepare()"){
            //     eprintln!("Script generation failed in play.prepare() called in main()");
            // }

            // println!("Script generation failed in play.prepare() called in main()");
            return ReturnWrapper::new(error.into());
        }
    }

    ReturnWrapper::new(SUCCESS.into())
}

fn usage(prog_name : &String) {
    let mut writer = std::io::stdout().lock();

    //All command line outputs including success and fail depends on match result, instead of providing extra arguments to writeln!()
    //Very confused since we are getting rid of println! and eprintln! to use writeln!, then what is the point of matching writeln!, which already takes in some error messages..?
    let _ = writeln!(&mut writer, "usage: {} <script_file_name>, [whinge]", prog_name);
        // eprintln!("usage: {} <script_file_name>, [whinge]", prog_name);

    
    
    // println!("usage: {} <script_file_name>, [whinge]", prog_name);
}

fn parse_args(config_arg : &mut String) -> Result<(), u8> {

    let mut vec = Vec::new();

    for arg in env::args(){

        vec.push(arg.to_string());

    }

    // checks if we have proper number of arguments when running the program.
    if vec.len() < MIN_ARGS || vec.len() > MAX_ARGS {

        usage(&vec[PROG_NAME]);

        return Err(BAD_ARGS);
    }

    if vec.len() == MAX_ARGS && vec[OPT_ARG] != "whinge" {

        //prints usage and return an error if the third argument is invalid
        usage(&vec[PROG_NAME]);

        return Err(BAD_ARGS);
    }

        *config_arg  = vec[CONFIG_NAME].clone();

        //This if statement checks whether 'whinge' is passed as a third argument
        //IS_WHINGE variable, if set to true, prints out warning message when we have problems parsing congfiguration files
        //or each lines in text files.
        if vec.len() == MAX_ARGS && vec[OPT_ARG] == "whinge" {

            lab3::declarations::IS_WHINGE.store(true, Ordering::SeqCst);

        }

        Ok(())
}
