use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, req: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Could not parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, handler: impl Handler) {
        match TcpListener::bind(&self.addr) {
            Ok(listener) => {
                println!("serving on {}", self.addr);
                self.listen_for_connections(listener, handler)
            }
            Err(e) => match e {
                //error type??
                _ => println!("Error: {}", e),
            },
        }
    }

    fn listen_for_connections(self, listener: TcpListener, mut handler: impl Handler) {
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Received buffer -> {}", String::from_utf8_lossy(&buf));
                            //use &buf[..] to change [u8; 1024] into &[u8]
                            let resp = match Request::try_from(&buf[..]) {
                                Ok(req) => handler.handle_request(&req),
                                Err(e) => handler.handle_bad_request(&e),
                                // Ok(req) => Response::new(
                                //     StatusCode::Ok,
                                //     Some("<h3>Toets 1 2 3</h3>".to_string()),
                                // ),
                                // Err(_) => Response::new(StatusCode::BadRequest, None),
                            };

                            if let Err(e) = resp.send(&mut stream) {
                                println!("Could not send response: {}", e);
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
