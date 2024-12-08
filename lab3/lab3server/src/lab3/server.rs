//server.js
//Author : DK Kim, donggyukim@wustl.edu
//creates server struct and necessary functions for server cargo
use std::net::TcpListener;
use std::io::{Write, Read};
use std::sync::atomic::{AtomicBool,Ordering};
use std::fs::File;
use std::thread;

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

    
    //a function that opens a server using ip address and a port number
    pub fn open(&mut self, str_slice : &str) {

        match TcpListener::bind(str_slice) {

            Ok(listener) => {
                
                self.listener = Some(listener);
                self.listener_addr = str_slice.to_string();
                println!("waiting for connections at address : {}", str_slice);
            },
            Err(e) => {
                let mut writer = std::io::stdout().lock();
                let _ = writeln!(
                    &mut writer,
                    "Failed to bind to address {}: {}",
                    str_slice, e
                );
            }
        }

        
    }

    //a function that runs a server, and closes if a client send 'quit' token
    pub fn run(&mut self) {

        while !CANCEL_FLAG.load(Ordering::SeqCst) && self.listener.is_some() {
            println!("Server running....");

            
            //reference : https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.accept
            let (mut stream, addr) = match self.listener.as_mut().unwrap().accept() {
                Ok((stream, addr)) => {
                    println!("new client: addr : {:?}", addr);
                    (stream, addr) 
                }
                Err(e) => {
                    println!("couldn't get client: {:?}", e);
                    continue; 
                }
            };


            println!("Connection received from {}", addr);

            if CANCEL_FLAG.load(Ordering::SeqCst) {
                return;
            };

            
            let _ =  thread::spawn(move || {
                
                // DK - using a const value kept return u8/usize related errors in this line and whenever buffer was used in the rs file.
                // So I intentionally left a hardcoded buffer initialization to make this code run.

                let mut buffer = [0; 100];
                let token = match stream.read(&mut buffer){
                    Ok(token) => {
                        token
                    }
                    Err(e) => {
                        println!("error while getting a token : {}", e);
                        return; 
                    }
                };

                //reference : https://stackoverflow.com/questions/19076719/how-do-i-convert-a-vector-of-bytes-u8-to-a-string
                let token_to_string = match std::str::from_utf8(&buffer[..token]){
                    Ok(token_to_string) => {
                        token_to_string
                    }
                    Err(e) => {
                        println!("error while parsing a token to a string : {}", e);
                        return; 
                    }
                };

                // If the received token is quit, set CANCEL_FLAG to true and quit the program.
                if token_to_string == "quit" {
                    CANCEL_FLAG.store(true, Ordering::SeqCst);
                    println!("Quitting the program due to quit token");
                    return;
                };

                // After passing all the prereqs above, we now check any expansion tokens to prevent file directory issues.
                if token_to_string.contains('/') || token_to_string.contains('\\') || token_to_string.contains("..") || token_to_string.contains('$') {
                    println!("file name check failed for token: {}", token_to_string);
                    return;
                };


                let title :String = token_to_string.to_string();
                let _ = match File::open(title){
                    Ok(title) => title,
                    Err(_) => {
                        println!("cannot open such a file");
                        return;
                    }
                };   
            
                match std::str::from_utf8(&buffer) {
                    Ok(string) => println!("The string: {}", string),
                    Err(e) => println!("Error while converting bytes to a string {}", e),
                }

    
                
            });

        } 
    }
}