use std::fs::read;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let listener_result = TcpListener::bind("127.0.0.1:4200")?;

    for stream in listener_result.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}


fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    println!("Received request\n");

    match stream.set_read_timeout(Some(Duration::from_millis(100))) {
        Ok(()) => 0,
        Err(_) => 1
    };

    let lines = match read_headers(&mut stream) {
        Ok(result) => result,
        Err(_) => {
            println!("error");
            stream.shutdown(Shutdown::Both)?;
            panic!("Failed to read request")
        }
    };

    for line in lines {
        println!("{}", line);
    }

    send_response(&mut stream)?;

    // stream.shutdown(Shutdown::Both)?;
    println!("Request processed\n");
    Ok(())
}

fn send_response(stream: &mut TcpStream) -> std::io::Result<()> {
    let response = "HTTP/1.1 200 OK\n\n <html><body><h1>Hello world</h1><p>Welcome to Rust</p></body></html>";
    let bytes = response.as_bytes();
    return match stream.write(bytes) {
        Ok(_) => Ok(()),
        Err(_) => panic!("")
    };
}

fn read_headers(stream: &mut TcpStream) -> Result<Vec<String>, ()> {
    let mut current_line: Vec<u8> = Default::default();
    let mut lines: Vec<String> = Default::default();
    let newline: u8 = 10;
    let mut buf: [u8; 1] = Default::default();

    let mut reader = BufReader::new(stream);


    loop {
        let foo = match reader.read(&mut buf) {
            Ok(num) => num,
            Err(_) => 0
        };

        if foo == 0 {
            break;
        }
        let char = buf[0];
        if char != newline {
            current_line.push(char);
            continue;
        } else {
            if current_line.len() == 1 {
                break;
            }
        }
        let mut copy = current_line.clone();
        let s = match std::str::from_utf8(&copy) {
            Ok(v) => v,
            Err(e) => Default::default(),
        };
        current_line.clear();
        lines.push(s.to_owned());
    }
    Ok(lines.to_owned())
}


