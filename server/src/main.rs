use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;
fn main() {
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bin");//Allows for the TCP listener to instantiate,
    //expect is just a catch that fails when the client was not able to grab the port that it
    //wanted to have.
    server.set_nonblocking(true).expect("failed to initializa non-blocking");//makes sure that the TCPlistener is non blocking and will only react with a message received in the port.
    //This prevents the whole client from freezing up while waiting for something to come into the
    //port
    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<String>();
    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);  
            let tx = tx.clone();
            clients.push(socket.try_clone().expect("failed to clone client"));
        } 
    
    }
}
