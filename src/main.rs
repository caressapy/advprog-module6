use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on 127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established!");
                handle_connection(stream);
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.expect("Failed to read line"))
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    let status_line = "HTTP/1.1 200 OK";
    
    // Pastikan file `hello.html` ada di direktori yang sama dengan `main.rs`
    let contents = fs::read_to_string("hello.html").unwrap_or_else(|_| "File not found".to_string());
    let length = contents.len();
 
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
 
    stream.write_all(response.as_bytes()).unwrap();
}
