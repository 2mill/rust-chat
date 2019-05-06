use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;
fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client.set_nonblocking(true).expect("failed to initiate non-blocking");
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buff = vec! [0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into 
            } 
        
        }
    })
}
