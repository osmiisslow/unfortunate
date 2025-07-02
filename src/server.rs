use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

// read the contents of the stream to the buffer of size 1024
// remove all trailing null using .trim_matches()
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let result = str::from_utf8(&buffer).unwrap()
        .trim_matches('\0');
    let lines: Vec<_> = result.lines().collect();
    println!("{lines:#?}");

    let (status_line, contents) = ("HTTP/1.1 200 OK", ":3");

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

pub fn start_server(port: i32) {
    println!("hosting server on port :{port}");
    let addr = format!("127.0.0.1:{port}");
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}