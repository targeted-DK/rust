use std::collections::BTreeSet;
use std::collections::HashSet;
use std::collections::BTreeMap;

fn main() {
    // let a : String = String::from("Thanks, Rosencrantz and gentle Guildenstern.");

    // let result: Vec<String> = a.split_whitespace().map(str::to_string).collect();
    // println!("{:?}", result);


    // let bt_result : BTreeSet<String>  = a.split_whitespace().map(str::to_string).collect();
    // println!("{:?}", bt_result);

    // let hash_result : HashSet<String> =  a.split_whitespace().map(str::to_string).collect();
    // println!("{:?}", hash_result);


    let b :  String = String::from("She sells sea shells by the sea shore.");
    // let result: Vec<String> = b.split_whitespace().map(str::to_string).collect();
    // println!("Vector : {:?}", result);


    // let bt_result : BTreeSet<String>  = b.split_whitespace().map(str::to_string).collect();
    // println!("BTreeSet : {:?}", bt_result);

    // let hash_result : HashSet<String> =  b.split_whitespace().map(str::to_string).collect();
    // println!("HashSet : {:?}", hash_result);

    let mut c : BTreeMap<usize, String> = BTreeMap::new();

    // for (i, item) in b.split_whitespace().map(str::to_string).collect::<Vec<_>>().iter().enumerate() {
    //     c.insert(i, item.to_string());
    // }
    // println!("BTreeMap : {:?}", c);

    let mut d : BTreeMap<String, usize> = BTreeMap::new();

    for (i, item) in b.split_whitespace().map(str::to_string).collect::<Vec<_>>().iter().enumerate() {
        d.insert(item.to_string(),i);
    }
    println!("BTreeMap : {:?}", d);


}

