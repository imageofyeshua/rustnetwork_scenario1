use std::net::{TcpListener, TcpStream};

fn main() {
    let _stream = TcpStream::connect("localhost:3000").unwrap();
}
