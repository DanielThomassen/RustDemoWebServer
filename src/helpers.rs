#![allow(dead_code)]
use std::net::TcpStream;
use std::io::Write;



pub fn write_string(stream: &mut TcpStream, value: &str) {
    match stream.write(value.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            println!("Failed to write value to stream:{0}, {1}", value,e);
            ()
        }
    }
}

pub fn byte_array_to_string(val: &[u8]) -> String {
    return match std::str::from_utf8(val) {
        Ok(v) => v,
        Err(_) => Default::default(),
    }.to_owned();
}

pub fn ends_with(v: &str, end: &str) -> bool {
    if v.len() < end.len() {
        return false;
    }

    let start = v.len() - end.len();

    let p = &v[start..];
    
    p == end
    
}

pub fn get_extension(v: &str) -> Result<&str, ()> {
    let mut i = v.len();

    let min = usize::try_from(0).unwrap();
    while i >= min {
        let ch_opt = v.chars().nth(i);

        if ch_opt.is_some() {
            let ch = ch_opt.unwrap();
            if ch == '.' {
                return Ok(&v[i..]);
            }
        }
        if i == 0 {
            break;
        }
        i = i-1;
    }
    return Err(());
}