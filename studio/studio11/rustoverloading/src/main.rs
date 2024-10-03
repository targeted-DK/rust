use std::fmt;
use std::ops::AddAssign;
use std::ops::SubAssign;





#[derive(Copy,Clone,Debug)] 
enum Locations {
    North,
    East,
    South,
    West,
}

#[derive(Copy,Clone,Debug)] 
enum Rotations {
    Left, 
    Right
}



trait Mover {
    fn advance(&mut self, distance :isize);
}

trait Turner {
    fn turn(&mut self, r: Rotations);
}

#[derive(Debug)]
struct Car {
    name : String,
    pos : Position,
    location : Locations,
}

#[derive(Copy,Clone,Debug)] 
struct Position {
    x : isize,
    y : isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Position {
        Position {x,y}
    }
}

impl Car {
    fn new(x: isize, y: isize) -> Car  {
        Car {
            name : "Lightning".to_string(),
            pos : Position::new(x,y),
            location : Locations::North,
            }
    }

    pub fn home(&mut self)  {
        self.pos.x -= self.pos.x;
        self.pos.y -= self.pos.y;
    }
    
    
}


impl AddAssign for Position {
    //reference : https://doc.rust-lang.org/std/ops/trait.AddAssign.html
    fn add_assign(&mut self, other:Self){

        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }

}

impl SubAssign for Position {
    fn sub_assign(&mut self, other:Self){
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}
impl Mover for Car {
    fn advance(&mut self, distance :isize){
        
        match self.location {
            Locations::North => {self.pos.y += distance},
            Locations::South => {self.pos.y -= distance},
            Locations::East  => {self.pos.x += distance},
            Locations::West  => {self.pos.x -= distance},
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


// fn figure_eight<T: Mover + Turner + std::fmt::Debug> (c: &mut T){
//     c.advance();
//     println!("{:?}", c);
//     c.turn(Rotations::Left);
//     c.advance();
//     println!("{:?}", c);
//     c.turn(Rotations::Left);
//     c.advance();
//     println!("{:?}", c);
//     c.turn(Rotations::Left);
//     c.advance();
//     println!("{:?}", c);
//     c.advance();
//     println!("{:?}", c);
//     c.turn(Rotations::Right);
//     c.advance();
//     println!("{:?}", c);
//     c.turn(Rotations::Right);
//     c.advance();
//     println!("{:?}", c);
//     c.turn(Rotations::Right);
//     c.advance();
//     println!("{:?}", c);


// }



fn main() {

   let mut test  = Car::new(3,4);

   println!("{:?}", test);

   test.turn(Rotations::Left);
   test.advance(1);
   test.turn(Rotations::Right);
   test.advance(5);

   println!("{:?}", test);

   test.home();
   println!("{:?}", test);

}
