
// struct Test {
//     value : String,
// }

// impl Test {
//     fn new(input : &String) -> Test {
//         Test{
//             value : input.to_string(),
//         }
//     }
// }


// impl Drop for Test {
//     fn drop(&mut self){
//         print!("Dropping {}", self.value);
//         if !self.value.is_empty() {
//             self.value = "".to_string();
//         };
//         println!("")
//     }
// }

#[derive(Default, Clone)]
struct Test<T: std::fmt::Display , S: std::fmt::Display > {
    value : T,
    second_value : S, 
}

impl<T: std::fmt::Display + Clone , S: std::fmt::Display + Clone > Test<T, S> {
    fn new(first : &T, second : &S) -> Test<T, S> {
        Test{
            value : first.clone(),
            second_value : second.clone(),
        }
    }
}

impl<T: std::fmt::Display + Clone  , S: std::fmt::Display + Clone > Test<T, S> {
    fn display(&self)  {
        println!("Two values of this struct is : {}, {}", &self.value, &self.second_value)
    }
}

// impl<T: std::fmt::Display> Test<T> {
//     fn display_value(&self) {
//         println!("Displaying value: {}", self.value);
//     }
// }
// #[cfg(oldexercise)] {
    impl<T: std::fmt::Display  , S: std::fmt::Display > Drop for Test<T, S> {
        fn drop(&mut self){
            println!("Struct dropped")
        }
    }
// }




fn main() {
    let mut a: Test<String, u16>  = Test::new(&Default::default(), &15u16 );
    let mut b: Test<String, u16>  = Test::new(&"b".to_string(), &Default::default() );

    a.display();
    b.display();
    // b.display_value();
    // println!{"{:?}", a};
    // println!{"{:?}", b};

    // a =  Test::new(&String::from("aa"));
    

}


