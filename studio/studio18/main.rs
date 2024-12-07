use std::thread;
use std::env;
use std::io::Write;
use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
use std::net::TcpListener;
use std::io::Read;
use std::env::args;
use std::net::TcpStream;
use std::thread::spawn;
use std::net::Shutdown;
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicBool;

static B :AtomicBool = AtomicBool::new(false);

fn main() {

        let adr = "127.0.0.1:7878";
        let listener = TcpListener::bind(adr).unwrap();


        let mut buffer = [0; 128];

        let connection_handle = spawn(move || {
            loop {
            let (mut stream, addr) = listener.accept().unwrap();
            println!("connection received from {}", addr);
            println!("{:?}", stream.read(&mut buffer));

            if B.load(Ordering::SeqCst) {
                break;
            }
            }
        });

        let mut stream = TcpStream::connect(adr).unwrap();
        let args: Vec<String> = env::args().collect();

        let message = args.join(" "); 

        stream.write_all(message.as_bytes());

        stream.shutdown(Shutdown::Both).expect("shutdown call failed");
        B.store(true,Ordering::SeqCst);
        let mut stream = TcpStream::connect(adr).unwrap();

        connection_handle.join().unwrap();
    

        #[cfg(oldexercise)] {
        let mut thread_handles = vec![];


        for arg in env::args().skip(1){
    
    
        thread_handles.push(
            thread::spawn(move || println!("{}",arg))
            );
        }
    
        for handle in thread_handles { 
            let _ =  handle.join();
        }
    }

        #[cfg(oldexercise5)] {
                let handle = thread::spawn(||{ println!("Hello from a child thread");});


                handle.join().unwrap();


                let mut thread_handles = vec![];


                for arg in env::args().skip(1){


                    thread_handles.push(
                        thread::spawn(move || arg)
                        );
                    }


                    for handle in thread_handles { 
                        let name =  handle.join().unwrap();
                

                        match File::open(name) {
                            Ok(file) => {
                                let reader = BufReader::new(file);
                                
                                for line in reader.lines() {
                                    match line {
                                    
                                        Ok(line) =>  if !line.is_empty(){
                                            println!("{}", line)
                                        },
                                        Err(e) => println!("{}", e),
                                    }
                                }
                            },
                            Err(e) => println!("{}", e),
                    }
                
            }
        }
}
