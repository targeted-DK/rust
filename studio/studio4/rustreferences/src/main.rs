fn main() {
  
    if cfg!(oldexercise1_5) { 
        let a = 0;
        let b = &a;
        let c = &b;

        println!("{}", a);
        println!("{}", b);
        println!("{}", c);

        // let d = b==*c;
        let d = *b == **c;
        println!("{}", d);
    }

    // if cfg!(oldexercise6_8) { 
    //     let mut a = 0;
    //     let mut b = &a;
    //     // let c = &b;

    //     // println!("a: {}", a);
    //     println!("b before: {}", b);
    //     b = &2;
       
    //     println!("b after: {}", b);
    //     // println!("b before: {}", a);
    //     // println!("b after: {}", b);
    //     // println!("c: {}", c);

    //     // let d = b==*c;
    //     // let d = *b == **c;
    //     // println!("d: {}", d);
    // }

    let mut a = "test".to_string();
    {
        let b = &mut a;
        println!("b before : {}", b);
        *b = "test2".to_string();
        println!("b after : {}", b);
    }




    
}
