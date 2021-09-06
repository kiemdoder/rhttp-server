use crate::http::Request;
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        match TcpListener::bind(&self.addr) {
            Ok(listener) => {
                println!("serving on {}", self.addr);
                self.listen_for_connections(listener)
            }
            Err(e) => match e {
                //error type??
                _ => println!("Error: {}", e),
            },
        }
    }

    fn listen_for_connections(self, listener: TcpListener) {
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Received buffer -> {}", String::from_utf8_lossy(&buf));
                            //use &buf[..] to change [u8; 1024] into &[u8]
                            match Request::try_from(&buf[..]) {
                                Ok(req) => {
                                    dbg!(req);
                                }
                                Err(e) => println!("Could not parse request: {}", e),
                            }
                        }
                        Err(e) => println!("Cound not read from connection: {}", e),
                    }
                }
                Err(e) => {
                    println!("Connection failed: {:?}", e);
                    continue;
                }
            }
        }
    }
}
