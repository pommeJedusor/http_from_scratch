use core::str;
use std::error::Error;
use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};

use std::collections::HashMap;

struct HttpResponse {
    headers: HashMap<String, String>,
    content: String,
}

fn create_header(method: &str, params: HashMap<&str, &str>) -> String {
    let mut result = String::from(format!("{method} / HTTP/1.1\r\n"));

    for (key, value) in &params {
        result.push_str(&format!("{key}: {value}\r\n"));
    }

    result.push_str("\r\n");
    return result;
}

fn headers_to_hashtable(headers: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut new_headers: HashMap<String, String> = HashMap::new();
    for (i, line) in headers.split("\n").enumerate() {
        // first line
        if i == 0 {
            let mut words = line.split(" ");
            let http_version = match words.nth(0) {
                Some(x) => Ok(x),
                None => Err("http version not found"),
            }?
            .trim();
            let status = match words.nth(0) {
                Some(x) => Ok(x),
                None => Err("http status not found"),
            }?
            .trim();
            let mut status_message = words.collect::<Vec<&str>>().concat();
            // remove the last char '\r'
            status_message.remove(&status_message.len() - 1);
            new_headers.insert("status".to_string(), status.to_string());
            new_headers.insert("http-version".to_string(), http_version.to_string());
            new_headers.insert("status-message".to_string(), status_message);
            continue;
        }

        let index = line[..line.len() - 1]
            .find(":")
            .expect(&format!("error line {i}\nline: {line:?}"));

        let parameter = line[0..index].trim().to_lowercase();
        let value = line[index + 1..].trim();
        new_headers.insert(parameter.to_string(), value.to_string());
    }

    Ok(new_headers)
}

fn get(url: &str, params: HashMap<&str, &str>) -> Result<HttpResponse, Box<dyn Error>> {
    let mut stream = TcpStream::connect(url)?;
    let header = create_header("GET", params);
    stream.write(header.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    stream.shutdown(Shutdown::Both)?;

    let split_index = response.find("\r\n\r\n").expect("invalid headers");
    let headers = headers_to_hashtable(&response[0..split_index])?;
    let content = &response[split_index + 4..];

    Ok(HttpResponse {
        headers,
        content: content.to_string(),
    })
}

fn main() {
    let _ = match get("localhost:8000", HashMap::from([("Host", "localhost")])) {
        Ok(x) => println!("headers: {:#?}\ncontent:\n{}", x.headers, x.content),
        Err(x) => println!("error: {x:?}"),
    };
}
