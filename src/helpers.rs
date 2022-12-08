use std::net::TcpStream;
use std::io::Write;

pub fn write_string(stream: &mut TcpStream, value: &str) {
    match stream.write(value.as_bytes()) {
        Ok(s) => s,
        Err(_) => panic!("Failed to write value to stream")
    };
}

pub fn byte_array_to_string(val: &[u8]) -> String {
    return match std::str::from_utf8(val) {
        Ok(v) => v,
        Err(_) => Default::default(),
    }.to_owned();
}
