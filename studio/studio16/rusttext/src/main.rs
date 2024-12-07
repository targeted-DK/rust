fn main() {
    let a = "Hello, world!";

    let b = a.chars().filter(|x| x.is_ascii_uppercase()).count();

    let c = a.len() - b;

    println!("{}", a);
    // println!("{}", b);
    // println!("{}", c);

    let d : Vec<_> = a.chars().rev().collect();

    println!("{}", a);
    println!("{:?}", d);

    let e : String = "kayak".chars().collect();

    let f : String = "kayak".chars().rev().collect();

    let g : String = "administration".chars().collect();

    let h : String = "administration".chars().rev().collect();
    println!("{}", isPalindrome(&e,&f));
    println!("{}", isPalindrome(&g,&h));

   
    filter("a7 6b b67a");

    
    filter_two("Madam, I'm Adam")

    let j  = "Eh! Ça va, la vache?";

    filter(&j);

    let k = "Ésope reste ici et se repose.";

    filter(&k);


}


fn isPalindrome(first : &str, second : &str) -> bool{

    println!("{}", first);
    println!("{}", second);

    return first == second
}

fn filter(first : &str) -> () {

    let mut forward = first.clone();
    let mut backward = first.clone();
  
    let mut fcheck = forward.chars().map(modify).filter(|letter| letter.is_ascii_alphanumeric()  ).collect::<Vec<_>>();
    let mut bcheck = backward.chars().rev().map(modify).filter(|letter| letter.is_ascii_alphanumeric() ).collect::<Vec<_>>();
    println!("{}", first);
    println!("{:?}", fcheck);
    println!("{:?}", bcheck);
    println!("{}",fcheck  == bcheck);


}

fn filter_two (first : &str) {


    let mut fcheck: Vec<_> = first.to_lowercase().chars().collect();
    let mut bcheck : Vec<_>= first.to_lowercase().chars().rev().collect();

    println!("{}", first);
    println!("{}", fcheck == bcheck)

}

fn modify(first : char) -> char{
    let converted = match first {
        'Ç' => 'c',
        'È'..='Ë' => 'e',
        _ => first
    };

    converted

}