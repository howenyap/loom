use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

pub struct Server;

impl Server {
    pub fn new() -> Self {
        Server
    }

    pub fn run(&self) {
        let address = concat!("127.0.0.1", ":", "3000");
        let listener = TcpListener::bind(address).expect("failed to spawn listener");

        let address = listener.local_addr().expect("failed to get socket address");
        println!("Binded to {address}");

        for stream in listener.incoming() {
            let stream = stream.expect("failed to read stream");
            self.handle_connection(stream);

            println!("Connection established!");
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream
            .read(&mut buffer)
            .expect("failed to read stream to buffer");

        let get = b"GET / HTTP/1.1\r\n";
        let (status_line, filename) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

        let contents = fs::read_to_string(filename).expect("failed to read response file");
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );

        stream
            .write(response.as_bytes())
            .expect("failed to write response to stream");
        stream.flush().expect("failed to flush to stream");

        let s = String::from_utf8_lossy(&buffer);
        println!("{s}");
    }
}
