mod rustmod {
    
use std::sync::atomic::AtomicUsize;

pub static B : AtomicUsize = AtomicUsize::new(2);

}

fn main() {
    pub use rustmod::B;
    pub use std::sync::atomic::Ordering;
    #[cfg(oldexercise2)] {
        pub const A : usize = 1;
    }

    #[cfg(oldexercise3)] {
        pub static A : usize = 1;
    }

    #[cfg(oldexercise4_5)] {
        pub static mut A : usize = 1;
        unsafe {
            println!("{}", A);
        }
    }

   
    println!("{}",  B.load(Ordering::SeqCst));
    //reference : https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html
    B.store(3, Ordering::SeqCst);

    println!("After stroing new value : {}",  B.load(Ordering::SeqCst))
    
}