use std::net::TcpStream;
use std::io::{BufReader, Read};

pub fn get_path(headers: &Vec<(String,String)>) -> Result<&str, &str> {
    if headers.len() == 0 {
        return Ok("/");
    }
    let mut split = headers[0].0.split(' ');

    split.nth(1).ok_or("/")
}

pub fn read_headers(stream: &mut TcpStream) -> Result<Vec<(String, String)>, ()> {
    let mut current_line: Vec<u8> = Default::default();

    const NEWLINE: u8 = 10;
    let mut buf: [u8; 1] = Default::default();

    let mut headers: Vec<(String, String)> = Vec::new();
    let mut reader = BufReader::new(stream);
    let mut is_header_name = true;


    let header_separator: u8 = 58;

    let mut header_name = String::from("");

    loop {
        let bytes_read = match reader.read(&mut buf) {
            Ok(num) => num,
            Err(_) => 0
        };

        if bytes_read == 0 {
            break;
        }

        let char = buf[0];

        if char == header_separator && is_header_name {
            is_header_name = false;
            header_name = crate::helpers::byte_array_to_string(&current_line);
            current_line.clear();
            continue;
        }

        if char != NEWLINE {
            current_line.push(char);
            continue;
        } else if current_line.len() == 1 && is_header_name {
            break;
        }
        let value = crate::helpers::byte_array_to_string(&current_line);

        if is_header_name {
            headers.push((value.to_owned(), String::from("")));
        } else {
            let key = header_name.clone();
            headers.push((key, value.to_owned()));
        }
        header_name = String::from("");
        current_line.clear();
        is_header_name = true;
    }
    Ok(headers)
}
