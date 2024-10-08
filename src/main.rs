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

fn get(url: &str, params: HashMap<&str, &str>) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(url)?;

    let header = create_header("GET", params);
    stream.write(header.as_bytes())?;

    // get the header
    let mut result: [u8; 8000] = [0; 8000];
    stream.read(&mut result)?;
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

fn main() {
    let _ = get("localhost:8938", HashMap::from([("Host", "localhost")]));
}
