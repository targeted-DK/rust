use std::fmt;

#[derive(Copy,Clone, Debug )] 
enum Locations {
    North,
    East,
    South,
    West,
}

#[derive(Copy,Clone, Debug)] 
enum Rotations {
    Left, 
    Right
}


trait Mover {
    fn advance(&mut self);
}

trait Turner {
    fn turn(&mut self, r: Rotations);
}

#[derive(Debug)] 
struct Car {
    name : String,
    x : isize,
    y : isize,
    location : Locations,
}

impl Mover for Car {
    fn advance(&mut self){
        match self.location {
            Locations::North => self.y += 1, 
            Locations::South => self.y -= 1,  
            Locations::East  => self.x += 1,  
            Locations::West  => self.x -= 1,  
        }
    }
    
}


impl Turner for Car {
    fn turn(&mut self, r: Rotations){
        match (self.location, r) {
            (Locations::North, Rotations::Right) => {self.location = Locations::East},
            (Locations::North, Rotations::Left) => {self.location = Locations::West},
            (Locations::East, Rotations::Right) => {self.location = Locations::South},
            (Locations::East, Rotations::Left) => {self.location = Locations::North},
            (Locations::South, Rotations::Right) => {self.location = Locations::West},
            (Locations::South, Rotations::Left) => {self.location = Locations::East},
            (Locations::West, Rotations::Right) => {self.location = Locations::North},
            (Locations::West, Rotations::Left) => {self.location = Locations::South},
        }
    }
}

impl Car {
    fn new() -> Car  {
        Car {
            name : "Lightning".to_string(),
            x : 0,
            y : 0,
            location : Locations::North,
            }
    }
    
    
}
impl fmt::Display for Locations {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Locations::North => write!(f, "North"),
            Locations::East => write!(f, "East"),
            Locations::South => write!(f, "South"),
            Locations::West => write!(f, "West"),
        }
    }
}


fn figure_eight<T: Mover + Turner + std::fmt::Debug> (c: &mut T){
    c.advance();
    println!("{:?}", c);
    c.turn(Rotations::Left);
    c.advance();
    println!("{:?}", c);
    c.turn(Rotations::Left);
    c.advance();
    println!("{:?}", c);
    c.turn(Rotations::Left);
    c.advance();
    println!("{:?}", c);
    c.advance();
    println!("{:?}", c);
    c.turn(Rotations::Right);
    c.advance();
    println!("{:?}", c);
    c.turn(Rotations::Right);
    c.advance();
    println!("{:?}", c);
    c.turn(Rotations::Right);
    c.advance();
    println!("{:?}", c);


}



fn main() {

   
    let test = &mut Car::new();

    #[cfg(oldexercise)]{
    println!("{},{},{},{}", test.name, test.x, test.y, test.location.to_string());
    test.advance();
    println!("{},{},{},{}", test.name, test.x, test.y, test.location.to_string());
    test.turn(Rotations::Left);
    println!("{},{},{},{}", test.name, test.x, test.y, test.location.to_string());
    }

    figure_eight(test);

}
