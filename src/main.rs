use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::time::Duration;
use servus::ThreadPool;

fn main() {
    env_logger::init();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    log::warn!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    
    let mut lines = reader.lines();
    let request_line = lines.next().unwrap().unwrap();

    let (filename, status_line) = match &request_line[..] {
        "GET / HTTP/1.1" => ("src/servus.html", "HTTP/1.1 200 OK"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("src/servus.html", "HTTP/1.1 200 OK")
        },
        _ => ("src/404.html", "HTTP/1.1 404 NOT FOUND")
    };

    let content = fs::read_to_string(filename).unwrap();
    let length = content.len();
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, length, content);
    stream.write_all(response.as_bytes()).unwrap();

}

