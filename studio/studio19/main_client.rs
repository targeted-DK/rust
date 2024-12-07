use async_std::net::TcpListener;
// use async_std::stream::StreamExt;
use async_std::task::block_on;
use async_std::stream::StreamExt;
// use async_std::



fn main() {


    // let bind_future = TcpListener::bind("127.0.0.1:7777");
    // bind_future.await;

    // let bind_future = TcpListener::bind("127.0.0.1:7777");
    // let response = match block_on(bind_future){
    //     Ok(_) => Ok("Server started".to_string()),
    //     Err(e) => Err(e),
    // };
    // println!("{:?}", response);

    let response = block_on(run_server());
    println!("{:?}", response.unwrap());

}
   

async fn run_server() -> std::io::Result<String> {

    // let connection = match TcpListener::bind("127.0.0.1:7777").await {
    //     Ok(_) => Ok("Server started".to_string()),
    //     Err(e) => Err(e),
    // };

    let listener = TcpListener::bind("127.0.0.1:7777").await?;
    println!("waiting for connections");
    let mut new_connections =  listener.incoming();
    while let Some(socket_result) = new_connections.next().await {
      
        let socket = socket_result?;
        println!("{:?}", socket);
        
    };
   
    Ok("Done".to_string())
    

}
