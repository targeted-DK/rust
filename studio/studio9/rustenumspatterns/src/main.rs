fn main() {
   
    #[cfg(oldexercise)] { 
    let a = ("test2", 2024);
    match a {
            (name, number) => {
                println!("matched ({}, {})", name, number);
            },
        
        }
    }

    #[cfg(oldexercise)] {
    use std::str::FromStr;

    let b = "31";
    match u8::from_str(&b) {
      Ok(val) => {
            println!("The variable represented a valid integer : {}", val);
            
        },

        Err(e) => {
            println!("The variable did not represent a valid integer : {}", e);
            }
        }
    }

    use std::str::FromStr;
    let b = "31";

    if let  Ok(val) = u8::from_str(&b) {
        
              println!("The variable represented a valid integer : {}", val);
        }
        else {
              println!("The variable did not represent a valid integer : {}", b);
        }
              
}
