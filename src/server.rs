use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

pub trait Server {
    fn run(&self);
}
pub struct ServerImpl {
    pub port: u16
}

impl ServerImpl {
    pub fn handle(&self, stream : &mut TcpStream) {
        let mut buf : [u8; 1024] = [0; 1024];
        stream.read(&mut buf).expect("Read is not successful");
        println!("{}", std::str::from_utf8(&mut buf).unwrap());
        stream.write(&buf).expect("Write is not successful");
    }
}

impl Server for ServerImpl {
    fn run(&self) {
        let listener = TcpListener::bind(std::format!("127.0.0.1:{}", &self.port));
        listener.unwrap().incoming().for_each(|x| { self.handle(& mut x.unwrap()) })
    }
}