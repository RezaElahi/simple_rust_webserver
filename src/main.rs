use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    
    let mut lines = reader.lines();
    let request_line = lines.next().unwrap().unwrap();

    let (filename, status_line) = if request_line == "GET / HTTP/1.1" {
        ("src/servus.html", "HTTP/1.1 200 OK")
    } else {
        ("src/404.html", "HTTP/1.1 404 NOT FOUND")
    };

    let content = fs::read_to_string(filename).unwrap();
    let length = content.len();
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, length, content);
    stream.write_all(response.as_bytes()).unwrap();

}

