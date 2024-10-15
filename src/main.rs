use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};

use std::collections::HashMap;
use std::str;

fn from_bytes_to_str(buf: &[u8]) -> &str {
    match str::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    }
}

fn create_header(method: &str, params: HashMap<&str, &str>) -> String {
    let mut result = String::from(format!("{method} / HTTP/1.1\n"));

    for (key, value) in &params {
        result.push_str(&format!("{key}: {value}\n"));
    }

    result.push_str("\n");
    return result;
}

fn bytes_headers_to_hashtable(headers: &[u8; 8000]) -> HashMap<&str, &str> {
    let mut new_headers: HashMap<&str, &str> = HashMap::new();
    let headers = from_bytes_to_str(headers);
    for (i, line) in headers.split("\n").enumerate() {
        // first line
        if i == 0 {
            let mut words = line.split(" ");
            let http_version = words.nth(0).unwrap().trim();
            let status = words.nth(0).unwrap().trim();
            new_headers.insert("status", status);
            new_headers.insert("http-version", http_version);
            continue;
        }
        // end of the headers
        if line == "\r" {
            break;
        }
        // if line not valid
        if !line.contains(":") {
            println!("error line {i}");
            println!("line: {:?}", line);
            new_headers.insert("error", "http header is invalid");
            continue;
        }

        // TODO: make the parameter case insensitive
        let index = line.find(":").unwrap();
        let parameter = line[0..index].trim();
        let value = line[index + 1..].trim();
        new_headers.insert(parameter, value);
    }

    new_headers
}

fn get(url: &str, params: HashMap<&str, &str>) -> std::io::Result<()> {
    match TcpStream::connect(url) {
        Ok(mut stream) => {
            let header = create_header("GET", params);
            stream.write(header.as_bytes())?;

            // get the header
            let mut result: [u8; 8000] = [0; 8000];
            stream.read(&mut result)?;
            let response_headers = bytes_headers_to_hashtable(&result);
            println!("{:#?}", response_headers);
            let result = from_bytes_to_str(&result);
            println!("{result}");

            // get the content
            let mut result: [u8; 8000] = [0; 8000];
            stream.read(&mut result)?;
            let result = from_bytes_to_str(&result);
            println!("{result}");

            stream.shutdown(Shutdown::Both)?;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn main() {
    let _ = match get("localhost:8000", HashMap::from([("Host", "localhost")])) {
        Ok(x) => println!("{:?}", x),
        Err(x) => println!("{:?}", x),
    };
}
