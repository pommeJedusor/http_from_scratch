use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};
use std::str;

fn from_bytes_to_str(buf: &[u8]) -> &str {
    match str::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    }
}

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8938")?;

    stream.write(b"GET / HTTP/1.1\nHost: localhost\n\n")?;

    let mut result: [u8; 8000] = [0; 8000];
    stream.read(&mut result)?;
    let result = from_bytes_to_str(&result);
    println!("{result}");

    let mut result: [u8; 336] = [0; 336];
    stream.read(&mut result)?;
    let result = from_bytes_to_str(&result);
    println!("{result}");

    stream.shutdown(Shutdown::Both)?;
    Ok(())
}
