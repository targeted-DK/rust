//reference : https://stackoverflow.com/questions/46388386/what-exactly-does-derivedebug-mean-in-rust
#[derive(Debug)]
struct Point<T> {
    a : T,
    b : T
}

impl<T> Point<T> {
    pub fn new(x : T, y: T) -> Point<T> {
        Point { a : x, b : y}
    }
    pub fn set(&mut self, x: T, y: T) {
        self.a = x;
        self.b = y;
    }
}


fn main() {
    // let mut p = Point {
    //     a : 10.0,
    //     b : 20.0
    // };

    // print!("p is (");
    // print!("{}," , p.a);
    // println!("{})" , p.b);

    // println!("{:?}", p);

    // p.a = 100.0;
    // p.b = 200.0;
    
    // println!("{:?}", p);
    // p = Point::new(1000.0, 2000.0);
    // println!("Values modified");
    // println!("{:?}", p);

    let mut test = Point::new(10, 20); 
    println!("{:?}", test);
    // test.a = 1000;
    // test.b = 2000;
    test.set(1000, 2000); 
    println!("Values modified");
    println!("{:?}", test);



}

