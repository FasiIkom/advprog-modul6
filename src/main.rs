use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream); 
    }
}

fn handle_connection(mut stream: TcpStream) {
    let request_line = parse_request(&mut stream);
    let (status_line, filename) = if request_line.contains("GET / ") {
        ("HTTP/1.1 200 OK", "assets/html/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "assets/html/oops.html")
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