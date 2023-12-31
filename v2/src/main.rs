use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs::File;
use std::collections::HashMap;

const DEBUG: bool = true;

fn log(msg: &str) {
    if DEBUG {
        println!("{}", msg);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    log("Request received");

    let mut endpoints: HashMap<&[u8], &str> = HashMap::new();
    endpoints.insert(b"GET / HTTP/1.1\r\n", "index.html");
    endpoints.insert(b"GET /style.css HTTP/1.1\r\n", "style.css");
    endpoints.insert(b"GET /github.svg HTTP/1.1\r\n", "github.svg");
    endpoints.insert(b"GET /linkedin.svg HTTP/1.1\r\n", "linkedin.svg");
    endpoints.insert(b"GET /gmail.svg HTTP/1.1\r\n", "gmail.svg");
    endpoints.insert(b"GET /discord.svg HTTP/1.1\r\n", "discord.svg");

    let mut response = String::new();

    for (endpoint, filename) in endpoints {
        if buffer.starts_with(endpoint) {
            response = generate_response(filename);
            log(format!("Sending response for {}", filename).as_str());
            break;
        }
    }

    if response.is_empty() {
        response = generate_response("404.html");
        log("Sending response for 404");
    }

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    log("Response sent");
}

fn generate_response(filename: &str) -> String {
    let mut response = String::new();
    let mut file = File::open(filename).unwrap();

    response.push_str("HTTP/1.1 200 OK\r\n");
    response.push_str("\r\n");

    file.read_to_string(&mut response).unwrap();
    response
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();
    log("Listening on port 80");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }

    log("Shutting down");
}
