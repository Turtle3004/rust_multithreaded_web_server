use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // bind() returns the new instance of TcpListener.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // incoming() returns a iterator that gives us a sequence of streams
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        //println!("Connection estabblished!");

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // Capacity -> 1024 bytes.

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
