use crate::server::{Server, ServerImpl};
use crate::client::{Client, ClientIntr};
use std::vec::Vec;
use std::env;
use std::env::args;

mod server;
mod client;

fn main() {
    let args : Vec<String> = env::args().collect();
    let (service, src_port, dst_port) = (&args[1], &args[2], &args[3]);

    match service.as_str() {
        "server" => {
            let server = ServerImpl{ port: src_port.parse::<u16>().unwrap() };
            server.run();
        }
        "client" => {
            let client = Client {
                src_port:  src_port.parse::<u16>().unwrap(),
                dst_port : dst_port.parse::<u16>().unwrap()
            };
            client.run();
        }
        other => {
            println!("Undefined service: {other}")
        }
    }
}