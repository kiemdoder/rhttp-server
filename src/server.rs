use std::net::{TcpListener, TcpStream};
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("serving on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
    }
}