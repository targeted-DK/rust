use std::env;
use std::sync::atomic::*;



fn main() {

    let count = env::args().collect::<Vec<_>>().len();

    
    for arg in env::args().collect::<Vec<_>>(){
        println!("This program's name is {}", arg);
    }
   
    // let mut atomic_true = AtomicBool::new(true);
    // check_cmd_line!((&MY_ATOMIC_BOOL, count), 1);
    // let mut atomic_second_true = AtomicBool::new(true);
    // check_cmd_line!((&MY_ATOMIC_BOOL, count), 1);
    check_cmd_line!();
    check_cmd_line!(false);
    check_cmd_line!();
   
}

pub static MY_ATOMIC_BOOL: AtomicBool = AtomicBool::new(true);

#[macro_export]
macro_rules! check_cmd_line {
    () => {
        if !MY_ATOMIC_BOOL.load(Ordering::SeqCst) {
            println!("provide either false or true");
            MY_ATOMIC_BOOL.store(true, Ordering::SeqCst); 
        }
    };
    
    (true) => {
        MY_ATOMIC_BOOL.store(true, Ordering::SeqCst);
    };
  
    (false) => {
        MY_ATOMIC_BOOL.store(false, Ordering::SeqCst);
    };
    ($left:expr, $right:expr) =>{
        match (&$left, &$right) {
           
            ((left_bool, left_val), right_val) => {
                
                    if *left_val != *right_val {
                        panic!(
                            "assertion failed: `(left == right)` (left: `{:?}`, right: `{:?}`)",
                            left_val, right_val
                        );
                    }

                if left_bool.load(Ordering::SeqCst) {
                    println!("boolean value is set to true");
                } else {
                    println!("boolean value is set to false");
                } 
        }
    };
}

    // ($left:expr) => {
    //     match (&$left) {
    //         if *left == true {

    //         }
    //     }
    //      println!("Hi {}, you are {} years old!", $name, $age);
    //      };

}