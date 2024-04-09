use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;

pub trait ClientIntr {
    fn run(&self);
}

pub struct Client {
    pub src_port : u16,
    pub dst_port : u16
}

impl ClientIntr for Client {
    fn run(&self) {
        let stream = TcpStream::connect(std::format!("127.0.0.1:{}", self.dst_port));
        let res = stream.unwrap().write("Message".as_bytes());
    }
}