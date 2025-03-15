use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use hello::ThreadPool;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let request_line = parse_request(&mut stream);
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "assets/html/hello.html"), 
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10)); ("HTTP/1.1 200 OK", "assets/html/hello.html") 
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "assets/html/OOPS.html"),
    };
    let response = build_response(status_line, filename);
    stream.write_all(response.as_bytes()).unwrap();
}

fn parse_request(stream: &mut TcpStream) -> String {
    let buf_reader = BufReader::new(stream);
    buf_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .next()
        .unwrap_or_default()
}

fn build_response(status_line: &str, filename: &str) -> String {
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
}