// use std::io;

fn main() {
   
    let a: isize = 15;
        let b : isize= 5;
        let c : isize= 5;


    #[cfg(oldexercise2)] { 
        let ans = a / (b-c);
        println!("{}", ans);
    }

    let nom = a;
    let denom = b - c;

    match divide_by_zero(&nom, &denom){
        Ok(ans) => {
            println!("Ok: {}", ans);
            return ()
        },
        Err(e) => {
            println!("Divide by zero Error: {}", e);
            // Err::<isize, isize>(e);
        }
    }
  
    // println!("{}",ans);
    // Ok(1)
    //  Ok((nom/denom).into())

    // println!("{}",divide_by_zero(&a,&b,&c))
}

fn divide_by_zero( nom : &isize,  denom : &isize) -> Result<isize, isize>{

    if *denom == 0{
        return Err(1);
    } else {
        Ok(nom/denom)
    }
}

