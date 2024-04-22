use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str::from_utf8;

pub trait Server {
    fn run(&self);
}
pub struct ServerImpl {
    pub port: u16
}

impl ServerImpl {
    pub fn handle(&self, stream : &mut TcpStream) {
        println!("HERE");
        let mut buf = [0; 1024];
        match stream.read(&mut buf) {
            Ok(bytes_read) => {
                println!("Read {} bytes: {:?}", bytes_read, from_utf8(&buf[..bytes_read]));
            },
            Err(e) => {
                eprintln!("Error reading from TcpStream: {}", e);
            },
        }
    }
}

impl Server for ServerImpl {
    fn run(&self) {
        let listener = TcpListener::bind(std::format!("127.0.0.1:{}", &self.port));
        listener.unwrap().incoming().for_each(|result| {
            match result {
                Ok(mut stream) => {
                    self.handle(&mut stream);
                }
                Err(err) => {
                    eprintln!("Error accepting incoming connection: {}", err);
                }
            }
        });
    }
}