use std::{
    fs::{File, OpenOptions},
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    path::Path,
};

use rand::seq::IteratorRandom;

use crate::http_request::HttpRequest;

fn get_quote(path: &Path) -> String {
    let file = File::open(path).unwrap();
    let file = BufReader::new(file);

    let quotes = file
        .lines()
        .map(|line| line.expect("couldn't read quote line!"));

    match quotes.choose(&mut rand::rng()) {
        Some(quote) => quote,
        None => "this server doesn't have quotes yet... add one! :3".to_string(),
    }
}

fn make_quote_file(path: &Path) {
    match path.exists() {
        true => println!("file {path:?} exists"),
        false => {
            println!("creating file {path:?}");
            File::create(path).unwrap();
        }
    }
}

fn add_quote_to_file(path: &Path, quote: &[u8]) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();
    file.write_all(quote).unwrap();
}

// read the contents of the stream to the buffer of size 1024
// remove all trailing null using .trim_matches()
fn handle_connection(mut stream: TcpStream, quotes_path: &Path) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let result = str::from_utf8(&buffer).unwrap().trim_matches('\0');
    let lines: Vec<_> = result.lines().collect();

    let request = HttpRequest::new(lines[0]);

    let (status_line, contents) = match request.method {
        "GET" => ("HTTP/1.1 200 OK", get_quote(quotes_path)),
        "POST" => {
            let mut quote = lines.last().unwrap().to_string();
            quote.push('\n');
            add_quote_to_file(quotes_path, quote.as_bytes());
            ("HTTP/1.1 201 Created", format!("added {quote}"))
        }
        _ => (
            "HTTP/1.1 501 Not Implemented",
            String::from("Failed to process request"),
        ),
    };

    let (status_line, contents) = match request.path {
        "/" => (status_line, contents),
        _ => (
            "HTTP/1.1 404 Not Found",
            String::from("uh oh! not found! :3"),
        ),
    };

    let (status_line, contents) = match request.version {
        "HTTP/1.1" => (status_line, contents),
        _ => (
            "HTTP/1.1 505 HTTP Version Not Supported",
            String::from("bad HTTP version :c"),
        ),
    };

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream
        .write_all(response.as_bytes())
        .unwrap_or_else(|e| eprintln!("unable to send response: {e}"));
}

pub fn start_server(port: i32) {
    let quotes_path = Path::new("quotes.txt");
    make_quote_file(quotes_path);
    println!("hosting server on port :{port}");
    let addr = format!("127.0.0.1:{port}");
    let listener =
        TcpListener::bind(addr).unwrap_or_else(|e| panic!("unable to bind to port:\n{e}"));

    for stream in listener.incoming() {
        let stream = stream.unwrap_or_else(|e| panic!("unable to process incoming request: {e}"));

        handle_connection(stream, quotes_path);
    }
}
