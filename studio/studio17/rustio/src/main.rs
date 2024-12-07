use std::io::Write;
use std::env;
use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
fn main() {
//    let a  = "Test";
//    let b = false;
//    let c = vec![1,2,3,4];

//    println!("{} , {} , {:?}", a, b, c);

//    let mut writer = std::io::stdout().lock();
//     let result = writeln!(&mut writer,"{} , {} , {:?}", a, b, c);
//     match result {
//         Ok(_) => println!("Done"),
//         Err(..) => println!("Error")
//     }


    let arg : Vec<_>   = env::args().collect();
    // println!("{}",arg[1]);

    // let mut f  = &arg[1];

    match arg.get(1) {

        Some(name) => {
            println!("Reading file: {}", name);
            
            match File::open(name) {
                Ok(file) => {
                    let reader = BufReader::new(file);
                    
                    for line in reader.lines() {
                        match line {
                           
                            Ok(line) =>  if !line.is_empty(){
                                println!("{}", line)
                            },
                            Err(e) => println!("{}", e),
                        }
                    }
                },
                Err(e) => println!("{}", e),
            }
        },
        None => println!("none"),
    }


}
