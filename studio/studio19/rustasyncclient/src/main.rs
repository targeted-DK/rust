use async_std::net::TcpStream;


fn main() {

    // matches on the result of a call to async_std::task::block_on(TcpStream::connect("127.0.0.1:7777"))

    match async_std::task::block_on(TcpStream::connect("127.0.0.1:7777") ){
        Ok(_) => {
            println!("{}", "connection successful");

        },
        Err(_) => {
            println!("{}", "connection failed");
        }


    }
    
}
