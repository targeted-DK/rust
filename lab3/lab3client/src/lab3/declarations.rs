//declarations.rs
//Author : DK Kim, donggyukim@wustl.edu
//files that contains necessary constants and types for this program
use std::sync::atomic::AtomicBool;

// pub type Play = Vec<(usize, String, String)>;
pub const FAIL_TO_SCRIPT : u8 = 2;
pub static IS_WHINGE : AtomicBool = AtomicBool::new(false);
pub const SUCCESS : u8 = 0;

//I added this to indicate that read_line() reach the end of line, returning a value of 0
pub const EOL_VAL : usize = 0;




