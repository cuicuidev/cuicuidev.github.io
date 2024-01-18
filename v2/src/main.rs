use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs::File;
use std::collections::HashMap;
use base64;

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
    endpoints.insert(b"GET /script.js HTTP/1.1\r\n", "script.js");
    endpoints.insert(b"GET /style.css HTTP/1.1\r\n", "style.css");
    endpoints.insert(b"GET /github.svg HTTP/1.1\r\n", "github.svg");
    endpoints.insert(b"GET /linkedin.svg HTTP/1.1\r\n", "linkedin.svg");
    endpoints.insert(b"GET /gmail.svg HTTP/1.1\r\n", "gmail.svg");
    endpoints.insert(b"GET /discord.svg HTTP/1.1\r\n", "discord.svg");
    endpoints.insert(b"GET /pisoscom.jpg HTTP/1.1\r\n", "pisoscom.jpg");

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

    let extension = filename.split('.').last().unwrap();
    log(format!("Extension: {}", extension).as_str());

    let is_binary = match extension {
        "html" | "css" | "js" | "svg" => false,
        _ => true,
    };

    if is_binary {
        let contents = read_binary_file(filename);
        let encoded_contents = base64::encode(&contents);

        response.push_str("HTTP/1.1 200 OK\r\n");
        response.push_str(format!("Content-Type: image/{}; charset=UTF-8\r\n", extension).as_str());
        response.push_str("Content-Transfer-Encoding: base64\r\n");
        response.push_str("\r\n");
        response.push_str(&encoded_contents);
    } else {
        let contents = read_text_file(filename);
        response.push_str("HTTP/1.1 200 OK\r\n");
        response.push_str("\r\n");
        response.push_str(&contents);
    }

    response

}

fn read_text_file(filename: &str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    contents
}

fn read_binary_file(filename: &str) -> Vec<u8> {
    let mut file = File::open(filename).unwrap();
    let mut contents = Vec::new();

    file.read_to_end(&mut contents).unwrap();
    contents
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
