use async_std::net::TcpListener;
use async_std::stream::StreamExt;
use std::iter::Iterator;
use std::io::Write;
use async_std::io::WriteExt;

async fn run_server() -> std::io::Result<String> {

    let connection =  TcpListener::bind("127.0.0.1:1234").await?;

    println!("{}", "bound successfully");
    let mut incoming = connection.incoming();
    
    while let Some(socket) = incoming.next().await {
      
        let socket = socket?;
        println!("{:?}", socket);
     
    }
    return Ok(String::from("done"));

    // match connection.await{


        
    //     Ok(_) => {
    //         let result = String::from("success in run_server");
    //         return Ok(result)
    //     },

    //     Err(e) => return Err(e),
    // }
    

    

   

}
fn main() {
    println!("Hello, world!");

//     let mut bind_future = TcpListener::bind("127.0.0.1:7777");
//     // bind_future.await;
//     match async_std::task::block_on(bind_future) {
//         Ok(_) => println!("Success"),
//         Err(err) => println!("{}", err)
//     }

run_server();
}
