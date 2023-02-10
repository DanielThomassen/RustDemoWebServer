use async_std::{net::TcpStream, io::ReadExt};
use async_std::io::BufReader;


pub fn get_path(headers: &Vec<(String,String)>) -> Result<&str, &str> {
    if headers.len() == 0 {
        return Ok("/");
    }
    let mut split = headers[0].0.split(' ');

    split.nth(1).ok_or("/")
}

pub async fn read_request(stream: &mut TcpStream) -> Result<(Vec<(String, String)>,String), ()> {
    let mut current_line: Vec<u8> = Default::default();

    const NEWLINE: u8 = 10;
    let mut buf: [u8; 1] = Default::default();

    let mut headers: Vec<(String, String)> = Vec::new();
    let mut reader = BufReader::new(stream);
    let mut is_header_name = true;

    let header_separator: u8 = 58;

    let mut header_name = String::from("");

    loop {
        let mut bytes_read = 0;
        let foo =  reader.read(&mut buf).await;        
        if foo.is_ok() {
            bytes_read = foo.unwrap();
        }
        
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
    println!("{} {}",headers[headers.len() -1].0,headers[headers.len() -1].1);
    current_line.clear();
    let mut i = 0;
    let mut body = String::new();
    
    loop {
        i += 1;
        let bytes_read = match async_std::io::timeout(core::time::Duration::from_millis(100), reader.read(&mut buf)).await {
            Ok(num) => num,
            Err(_) => 0
        };

        if bytes_read == 0 {
            break;
        }
        if buf[0] != NEWLINE {
            current_line.push(buf[0]);
            continue;
        }        
        body.push_str(&crate::helpers::byte_array_to_string(&current_line));
        current_line.clear();
    }
    if current_line.len() > 0 {
        body.push_str(&crate::helpers::byte_array_to_string(&current_line));
    }

    Ok((headers,body))
}
