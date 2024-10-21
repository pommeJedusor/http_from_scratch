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
    let mut result = String::from(format!("{method} / HTTP/1.1\n"));

    for (key, value) in &params {
        result.push_str(&format!("{key}: {value}\n"));
    }

    result.push_str("\n");
    return result;
}

fn bytes_headers_to_hashtable(
    headers: &[u8; 8000],
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut new_headers: HashMap<String, String> = HashMap::new();
    let headers = str::from_utf8(headers)?;
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
            new_headers.insert("status".to_string(), status.to_string());
            new_headers.insert("http-version".to_string(), http_version.to_string());
            continue;
        }
        // end of the headers
        if line == "\r" {
            break;
        }
        // if line not valid
        if !line.contains(":") {
            return Err(format!("error line {i}\nline: {line:?}").into());
        }

        let index = line
            .find(":")
            .expect("didn't find the ':' char even though we just checked before it was there?");
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

    // get the header
    let mut result: [u8; 8000] = [0; 8000];
    stream.read(&mut result)?;
    let response_headers = bytes_headers_to_hashtable(&result)?;
    let expected_length: usize = response_headers
        .get("content-length")
        .unwrap_or(&"0".to_string())
        .parse()?;

    // get the content
    let mut content = vec![0; expected_length];
    let length = stream.read(&mut content)?;
    if length != expected_length {
        return Err("length of the headers didn't correspond with the received header".into());
    }
    let content = str::from_utf8(&content)?;

    stream.shutdown(Shutdown::Both)?;

    let response = HttpResponse {
        headers: response_headers,
        content: content.to_string(),
    };

    Ok(response)
}

fn main() {
    let _ = match get("localhost:8000", HashMap::from([("Host", "localhost")])) {
        Ok(x) => println!("headers: {:#?}\ncontent:\n{}", x.headers, x.content),
        Err(x) => println!("error: {x:?}"),
    };
}
