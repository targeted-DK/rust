fn main() {
    
    let mut a : u8 = 1;
    let mut count : u8 = 0;

    //references : https://doc.rust-lang.org/std/primitive.u32.html
    //2.
    if cfg!(oldexercise) {
        loop {
        println!("{}", a);
        a += 1;
        } 
    }

    //3.
    if cfg!(oldexercise){
        while a.checked_mul(2).is_some() {
            
            a *= 2;
    
            println!("{}", a);
            }
        }

    //4. 
    //a.overflowing_mul(u8) returns a tuple of two values : (u32, bool) and .1 returns boolean value whether the function overflows or not
    if cfg!(oldexercise) { 
        while !a.overflowing_mul(2).1 {
            a *= 2;
            count += 1;
            println!("count : {}", count);
            println!("a : {}", a);           
        }
       
    }
  
   
    //5.
    if cfg!(oldexercise) {
        for _i in 0..8 {
            a  = a.saturating_mul(2);
            println!("count : {}", _i + 1);
            println!("a : {}", a);     
            }
    }

    //6.
    if cfg!(oldexercise) {
        for _i in 0..8 {
            a  = a.wrapping_mul(2);
            println!("count : {}", _i + 1);
            println!("a : {}", a);     
        }
    }   

    //7.
    let mut vec = Vec::new();
    vec.push(a);
    for _i in 0..8 {
        a  = a.wrapping_mul(2);
        println!("count : {}", _i + 1);
        println!("a : {}", a);     

        vec.push(a);
    }

    vec.sort();
    //reference : https://stackoverflow.com/questions/30320083/how-to-print-a-vec
    println!("{:?}", vec);
    
}
