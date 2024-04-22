use signal_hook::consts::SIGINT;
use std::io::prelude::*;
use std::io::{stdin};
use std::net::TcpStream;
use std::sync::{Arc};
use signal_hook::iterator::Signals;
use std::sync::atomic::{AtomicBool, Ordering};

pub trait ClientIntr {
    fn run(&self);
}

pub struct Client {
    pub src_port : u16,
    pub dst_port : u16,
    is_stopped : AtomicBool
}
impl Client {
    pub fn new(src_port : u16, dst_port : u16) -> Client {
        let client = Client {
            src_port: src_port,
            dst_port: dst_port,
            is_stopped: AtomicBool::new(false)
        };
        client.is_stopped.store(false, Ordering::Relaxed);
        client
    }

    // Passing an Arc to a static function guarantees that the client is allocated on the heap,
    // so we can share the reference to it with the spawned thread and be sure that the object
    // outlives the thread's scope. Otherwise, the object could be allocated on the stack
    pub fn run_signal_handler(client : Arc<Self>) {
        std::thread::spawn(move || {
            let signals = Signals::new([SIGINT]);
            for signal in signals.expect("Signal processing error").forever() {
                if signal == SIGINT {
                    client.is_stopped.store(true, Ordering::Relaxed);
                    break;
                };
            }
        });
    }
}

impl ClientIntr for Client {
    fn run(&self) {
        let dst = std::format!("127.0.0.1:{}", self.dst_port);
        let mut stream = TcpStream::connect(dst.as_str()).expect(
            std::format!("Cannot connect to the server {}", dst).as_str());
        let mut buffer = String::with_capacity(1024);

        loop {
            println!("Write a line that'll be sent to the server");

            buffer.clear();

            stdin().read_line(&mut buffer).expect("Cannot read a line");
            stream.write(buffer.trim().as_bytes()).expect("Cannot write a message");
        }
    }
}