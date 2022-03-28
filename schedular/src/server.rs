use deadliner_gui::new_path;
use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub fn run_server(exit: Arc<Mutex<bool>>) {
    let port = fs::read_to_string(new_path("port.txt")).unwrap();
    let addr = format!("127.0.0.1:{}", port);

    let listener = TcpListener::bind(&addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.expect("Couldn't establish a socket connecton!");

        // Pass the sender here to trigger shutdow
        let exit_tcp_listener = handle_connection(stream, Arc::clone(&exit));
        if exit_tcp_listener {
            break;
        }
    }
}

fn handle_connection(mut stream: TcpStream, exit: Arc<Mutex<bool>>) -> bool {
    let mut buffer = [0; 1024];
    stream
        .read(&mut buffer)
        .expect("Couldn't read the `TcpStream` buffer!");

    let req = Request::new(&buffer).parse();

    // hit any endpoint to check if the schedular is already running.
    stream
        .write(response(200, "OK", "", "").as_bytes())
        .expect("Couldn't write bytes to the stream!");

    stream
        .flush()
        .expect("Couldn't write all bytes to the stream!");

    if req.is_some() && req.unwrap().uri == "/shutdown" {
        let mut exit = exit.lock().unwrap();
        *exit = true;

        return true;
    }

    false
}

/// `Request` is a struct that takes the `buffer` from `TcpStream`
/// and can call methods on it to parse the request buffer to a `ParsedRequest`
/// which have all of the useful information you would like to deal with.
pub struct Request<'a> {
    buffer: &'a [u8],
}

/// `ParsedRequest` is a return type of the `parse` method on the `Request` struct
/// It contains all of the metadata extracted from request data buffer.
#[derive(Debug, PartialEq)]
pub struct ParsedRequest {
    method: String,
    uri: String,
    http_version: f64,
    body: String,
}

impl<'a> Request<'a> {
    /// Instantiate a new `Request` struct by providing a request buffer data as the only argument.
    pub fn new(buffer: &'a [u8]) -> Request<'a> {
        Request { buffer }
    }

    /// Parses the request buffer and returns a `Option<ParsedRequest>`
    /// In the `Some<ParsedRequest>` case means that it was able to parse the buffer successfully.
    /// In the `None` case it means that the buffer was malformed and it could't able to parse it.
    ///
    /// You can find all kind of useful data from the parsed request buffer like:
    /// - method
    /// - uri
    /// - http version
    /// - headers
    /// - body
    pub fn parse(&self) -> Option<ParsedRequest> {
        let req_str = String::from_utf8_lossy(self.buffer);

        let mut body = String::from("");

        let mut lines: Vec<&str> = req_str.lines().collect();

        let mut parts = lines[0].split(" ");

        let method = parts.next()?.to_string();
        let uri = parts.next()?.to_string();
        let http_version = parts
            .next()?
            .replace("HTTP/", "")
            .parse()
            .expect("Couldn't parse http version!");

        lines.remove(0);

        Some(ParsedRequest {
            body,
            http_version,
            method,
            uri,
        })
    }
}

/// A useful helper function for formatting the response string for easy re-use
/// It constructs a well-formatted response string using the provided arguments
/// `status`, `desc`, `headers` and the `body` of the response.
pub fn response(status: i32, desc: &str, headers: &str, body: &str) -> String {
    format!(
        "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
        status, desc, headers, body
    )
}
