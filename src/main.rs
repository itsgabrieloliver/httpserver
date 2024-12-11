use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;

use httpserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    let pool = ThreadPool::new(4);

    println!("Listening..");

    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => {
                pool.execute(|| handle_connection(stream));
            }
            Err(error) => {
                eprintln!("[ERROR] Failed to get TCP stream. {error}");
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(e)) => {
            eprintln!("Failed to read line from stream: {e}");
            return;
        }
        None => return,
    };

    println!("{request_line}");

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "html/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "html/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
