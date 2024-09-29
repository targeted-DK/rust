//declarations.rs
//Author : DK Kim, donggyukim@wustl.edu
//files that contains necessary constants and types for this program
use std::sync::atomic::AtomicBool;

pub type Play = Vec<(usize, String, String)>;
pub const MIN_ARGS : usize = 2;
pub const MAX_ARGS : usize = 3;
pub const PROG_NAME : usize = 0;
pub const CONFIG_NAME : usize = 1;
pub const OPT_ARG : usize = 2;
pub const BAD_ARGS : u8 = 1;
pub const FAIL_TO_SCRIPT : u8 = 2;
pub static IS_PROBLEM : AtomicBool = AtomicBool::new(false);




