fn main() {
   
    //2.
    if cfg!(oldexercise) {
        let a : u8 = 0;
        let b : u8 = 1;
        let c : u8 = b;

        println!("{}", a);
        println!("{}", b);
        println!("{}", c);
    }

    //5.
    let a : String = "test1".to_string();
    let mut b : String = "test2".to_string();
    let mut c : String = "test3".to_string();
 
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);

    b = c;
    c = "test4".to_string();

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);


    let mut vec = Vec::new();
    vec.push(a);
    vec.push(b);
    vec.push(c);

    println!("{:?}", vec);

    // let d = vec[vec.len() - 1];
    let d = vec.pop().unwrap();
    
    // match vec.pop() {
    //     d;
    // };

    println!("{}", d);


}
