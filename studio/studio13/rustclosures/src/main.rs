fn main() {
   
    let a : f64 = 0.0;

    
    // println!("{}", f64::cos(f64::sin(a)));

    // let result = || f64::cos(f64::sin(a));

    // println!("{}", result());

    // let obj = MyStruct::<f64>::new(a, f64::cos, f64::sin);

    // let result = || (obj.func_one)((obj.func_two)(obj.first));

    // println!("{}", result());

    let mut a : String = "Hello".to_string();
    
    let mut result =  || {
        a.push_str(", World!");
        println!("{:?}", a);
    };

    result();
    // println!("{:?}", result());
}

struct MyStruct<T> {
    first : T,
    func_one : fn(T) -> T,
    func_two : fn(T) -> T,
}

impl<T> MyStruct<T> {
    fn new(input : T, f1 : fn(T) -> T, f2:  fn(T)-> T) -> Self {
        MyStruct {
            first : input,
            func_one :f1,
            func_two :f2,
        }
    }
}