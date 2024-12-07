//return_wrapper.rs
//Author : DK Kim, donggyukim@wustl.edu
//creates a ReturnWrapper struct for a return value of main program of server 
use std::process::ExitCode;
use std::process::Termination;

pub const SUCCESS : u8 = 0;

pub struct ReturnWrapper {
    return_value : u8,
}

impl ReturnWrapper {
    pub fn new(value : u8) -> Self {
        ReturnWrapper {
            return_value : value,
        }
    }
}

impl Termination for ReturnWrapper {
    fn report(self) -> ExitCode {
        if self.return_value != SUCCESS {

            eprintln!("Error : {}", self.return_value);

        }
        return ExitCode::from(self.return_value);
    }

}

