//server.js
//Author : DK Kim, donggyukim@wustl.edu
//creates server struct and necessary functions for server cargo
use std::net::TcpListener;
use std::io::{Write, Read};
use std::io::read_to_string;
use std::sync::atomic::{AtomicBool,Ordering};
use std::fs::File;
use std::thread;
// use std::io;

static CANCEL_FLAG : AtomicBool = AtomicBool::new(false);


pub struct Server {
    pub listener : Option<TcpListener>,
    pub listener_addr : String
}


impl Server {
    pub fn new() -> Self {
        Server {
            listener : None,
            listener_addr : String::new(),
        }
    }

    
    fn open(&mut self, str_slice : &str) {

        match TcpListener::bind(str_slice) {

            Ok(listener) => {
                println!("waiting for connections");
                self.listener = Some(listener);
                self.listener_addr = str_slice.to_string();
            },
            Err(_) => {
                let mut writer = std::io::stdout().lock();
                let _ = writeln!(&mut writer,"There is an error while binding a str_slice to a TcpListener variable in server.open()");
            }
        }
    }

    fn run(&mut self) {

        //Instruction 10 - check bold instructions later
        while !CANCEL_FLAG.load(Ordering::SeqCst) && self.listener.is_some() {
            
            //reference : https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.accept
            let (mut stream, addr) = match self.listener.as_mut().unwrap().accept() {
                Ok((stream, addr)) => {
                    println!("new client: {addr:?}");
                    (stream, addr) 
                }
                Err(e) => {
                    println!("couldn't get client: {e:?}");
                    continue; 
                }
            };

            // let (mut stream, addr) = self.listener.unwrap().accept();


            println!("Connection received from {}", addr);

            if CANCEL_FLAG.load(Ordering::SeqCst) {
                return;
            };

            // let mut write_stream = match stream.try_clone(){
            //     Ok(stream) => {
            //         stream
            //     }
            //     Err(e) => {
            //         println!("error while creating a stream");
            //         return; 
            //     }
            // };

            
            thread::spawn(move || {

                let mut buffer = [0; 100];
                let token = match stream.read(&mut buffer){
                    Ok(token) => {
                        token
                    }
                    Err(e) => {
                        println!("error while getting a token");
                        return; 
                    }
                };
                //reference : https://stackoverflow.com/questions/72774370/rust-is-there-a-way-to-convert-bufreaderfile-to-a-string
                let token_to_string = match read_to_string(token.unwrap()){
                    Ok(token_to_string) => {
                        token_to_string
                    }
                    Err(e) => {
                        println!("error while parsing a token to a string");
                        return; 
                    }
                };

                let title = String::new();
                if token_to_string == "quit" {
                    CANCEL_FLAG.store(true, Ordering::SeqCst);
                    println!("Quitting the program due to quit token")
                    return;
                } 

                if token.contains('/') || token.contains('\\') || token.contains("..") || token.contains('$') {
                    println!("file name check failed for token: {}", token_to_string);
                    return;
                }


                title = token_to_string;
                let mut file = match File::open(title){
                    Ok(title) => title,
                    Err(_) => {
                        println!("cannot open such a file")
                        return;
                    }
                };
            


                println!("The bytes: {:?}", &buffer[..token]);

                // match stream.read(&write_stream) {

                // }

                // io::copy(&mut stream, &mut write_stream).expect("error in client thread : ");
                // println!("connection closed");

                
                // for stream in listener.incoming() {
                //     match stream {
                //         Ok(stream) => {
                //             handle_connection(stream);
                //         }
                //         Err(e) => { /* connection failed */ }
                //     }
                // }
                
            }).join();

        } 
    }
}

