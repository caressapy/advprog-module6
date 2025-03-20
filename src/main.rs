use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() -> () {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) -> () {
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let request_line: String = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename): (&str, &str) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "oops.html")
    };

    let contents: String = fs::read_to_string(filename).unwrap();
    let length: usize = contents.len();

    let response: String = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}