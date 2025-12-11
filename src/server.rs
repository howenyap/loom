use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};

use crate::http::Request;
use crate::threadpool::ThreadPool;

#[derive(Default)]
pub struct Server;

impl Server {
    pub fn new() -> Self {
        Server
    }

    pub fn run(&self) {
        let address = concat!("127.0.0.1", ":", "3000");
        let listener = TcpListener::bind(address).expect("failed to spawn listener");
        let pool = ThreadPool::new(8);

        let address = listener.local_addr().expect("failed to get socket address");
        println!("Binded to {address}");

        for stream in listener.incoming() {
            let stream = stream.expect("failed to read stream");

            pool.execute(|| Self::handle_connection(stream));

            println!("Connection established!");
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        let n = stream
            .read(&mut buffer)
            .expect("failed to read stream to buffer");

        let ok_pair = ("HTTP/1.1 200 OK", "hello.html");
        let error_pair = ("HTTP/1.1 404 NOT FOUND", "404.html");

        let (status_line, filename) = if let Ok(request) = Request::from_buffer(&buffer[..n]) {
            match request.uri() {
                "/" => ok_pair,
                "/sleep" => {
                    thread::sleep(Duration::from_secs(5));
                    ok_pair
                }
                _ => error_pair,
            }
        } else {
            error_pair
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
    }
}
