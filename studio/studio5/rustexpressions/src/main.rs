fn main() {
   
    if cfg!(oldexercise2) {
        println!("{}", 3+5*1);
    }

    if cfg!(oldexercise3_4){
    let a = 5;
    let b = 13;

        if a < b {
            println!("{}", a);
        } else {
            println!("{}", b);
        }
    }

    // let a = 5;
    // let b = 13;
    if cfg!(oldexercise5){
    let c = (23, 1);

    match c {
        (first, second) => println!("{}", if first > second {first} else {second})
        }
    }

    let mut vec  = Vec::new();

    for i in 0..10 {
        vec.push(i);
    }
    
    
    for val in &vec {
        println!("{}", val);
    }
    println!("{:?}", vec);
    

}
