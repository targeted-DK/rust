fn main() {

    #[cfg(oldexercise4)] {
    let a = "Hello, world!";

    let b = a.chars().filter(|x| x.is_ascii_uppercase()).count();

    let c = a.len() - b;

    println!("{}", a);

    let d : Vec<_> = a.chars().rev().collect();

    println!("{}", a);
    println!("{:?}", d);
    }

    #[cfg(oldexercise4)] {
    let e : String = "kayak".chars().collect();


    let g : String = "administration".chars().collect();

    println!("{}", is_palindrome(&e));
    println!("{}", is_palindrome(&g));
    }


    #[cfg(oldexercise5)] {
    println!("{}", is_palindrome("a7 6b b67a"));
    }

    #[cfg(oldexercise6)] {
    println!("{}", is_palindrome("Madam, I'm Adam"));
    }


    #[cfg(oldexercise7)] {
    println!("{}", is_palindrome_filter("Eh! Ça va, la vache?"));
    }

    println!("{}", is_palindrome_filter("Ésope reste ici et se repose."));

    

}


fn is_palindrome(first : &str) -> bool{

    let mut lowercased = first.to_lowercase();

    let mut firstcheck = lowercased.chars().filter(|letter| letter.is_ascii_alphanumeric()  ).collect::<Vec<_>>();
    let mut secondcheck = lowercased.chars().rev().filter(|letter| letter.is_ascii_alphanumeric() ).collect::<Vec<_>>();

    println!("{:?}", firstcheck);
    println!("{:?}", secondcheck);
    return firstcheck == secondcheck
}

fn is_palindrome_filter(first : &str) -> bool{

    let mut lowercased = first.to_lowercase();

    let mut firstcheck = lowercased.chars().map(modify).filter(|letter| letter.is_ascii_alphanumeric()  ).collect::<Vec<_>>();
    let mut secondcheck = lowercased.chars().rev().map(modify).filter(|letter| letter.is_ascii_alphanumeric() ).collect::<Vec<_>>();

    println!("{:?}", firstcheck);
    println!("{:?}", secondcheck);

    return firstcheck == secondcheck
}


fn modify(c : char) -> char{
    match c {
        //had to add lowercased version of accented c and e because my is_palindrome_filter lowercases input before appyling map().filter()......!!
        'Ç' | 'ç' => 'c',
        'È'..='Ë' => 'e',
        'è'..= 'ë' => 'e',
        _ => c
    }

}