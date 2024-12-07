fn main() {

    let mut container : Vec<bool> = Vec::new();

    //https://doc.rust-lang.org/std/iter/struct.Map.html
    let v : Vec<bool> = (0..99).map(|i|  {
        if (i % 2 == 0 && i > 2)  || i == 2  { true } else { false } 
    } ).collect();

    
    // (0..99).map((i % 2 == 0 && i > 2)  || i == 2)

    // for i in 0..=99 {

    //     let clo = || container.push
        
    //     // if (i % 2 == 0 && i > 2)  || i == 2 {
    //     //     container.push(true);
    //     // } else {
    //     //     container.push(false);
    //     // }
    // }


         let mut i = 3;
        let result = format!("{:10} ", i); 

        //https://stackoverflow.com/questions/72540172/how-can-i-filter-a-vector-on-an-index-in-rust
        let mut iterator = (&v).into_iter();
        while let Some(element) = iterator.next_chunk() {
            println!("{:?}", element)
            
        }
        // let filtered_vec :  Vec<_> = v.clone().into_iter().enumerate().filter(|&(index, boolval)| { if boolval == true  { true } else {false }}).collect();

       
        // if filtered_vec[i].1 == true {
        //     print!("{:10}", filtered_vec[i].0);
        // } else {
        //     print!("{:10}", "");
        // }
        

        // if (i + 1) % 10 == 0 {
        //     println!();
        // }
    }
