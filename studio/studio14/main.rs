fn main() {

    let mut container : Vec<bool> = Vec::new();

    //https://doc.rust-lang.org/std/iter/struct.Map.html
    #[cfg(oldexercise3)] {
          

        for i in 0..=99 {
            
            if (i % 2 == 1 && i > 2)  || i == 2 {
                container.push(true);
            } else {
                container.push(false);
            }
        }   

         for (i, &value) in container.iter().enumerate() {
            if value {
                    print!("{} ", i);
                }    
            }
    }


    #[cfg(oldexercise4)] {
    let v : Vec<bool> = (0..99).map(|i|  {
        if (i % 2 == 1 && i > 2)  || i == 2  { true } else { false } 
    } ).collect();

    for (i, &value) in v.iter().enumerate() {
        if value {
            print!("{} ", i);
        }
    }
    }    


    #[cfg(oldexercise5)] {
    let mut v : Vec<bool> = (0..99).map(|i|  {
        if (i % 2 == 1 && i > 2)  || i == 2  { true } else { false } 
    } ).collect();

    for i in 3..5 {
        if v[i] {
            for j in (0..v.len()) {
                if j % i == 0 {
                    v[j] = false; 
                }
               
            }
        }
    }

   
    for (i, &value) in v.iter().enumerate() {
        if value {
            print!("{} ", i);
        }
    }
    }
    #[cfg(oldexercise6)] {
    let mut v : Vec<bool> = (0..99).map(|i|  {
        if (i % 2 == 1 && i > 2)  || i == 2  { true } else { false } 
    } ).collect();


    (3..5).for_each({|i| 
        if v[i] {
            for j in (0..v.len()) {
                if j % i == 0 {

                    
                    v[j] = false; 
                }
               
            }
        }
    });

        for (i, &value) in v.iter().enumerate() {
            if value {
                print!("{} ", i);
            }
        }

    }


    //exercise 7.
    let mut v : Vec<bool> = (0..99).map(|i|  {
        if (i % 2 == 1 && i > 2)  || i == 2  { true } else { false } 
    } ).collect();


    (3..5).for_each({|i| 
        if v[i] {
            for j in (i.pow(2)..v.len()).step_by(i) {
                if j % i == 0 {

                    
                    v[j] = false; 
                }
            
            }
        }
    });

    for (i, &value) in v.iter().enumerate() {
        if value {
            print!("{} ", i);
        }
    }


    }
