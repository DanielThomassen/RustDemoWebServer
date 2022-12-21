use std::net::{Shutdown, TcpListener, TcpStream};
use std::time::Duration;
use crate::headers::get_path;
use crate::response::send_response;

mod headers;
mod response;
mod helpers;


fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");

    let address = "127.0.0.1:4200";

    let listener_result = TcpListener::bind(address)?;

    println!("Server listening on {}", address);

    for stream in listener_result.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}


fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    match stream.set_read_timeout(Some(Duration::from_millis(500))) {
        Ok(()) => 0,
        Err(_) => 1
    };

    let headers = match headers::read_headers(&mut stream) {
        Ok(result) => result,
        Err(_) => {
            stream.shutdown(Shutdown::Both)?;
            panic!("Failed to read request")
        }
    };

    if headers.len() == 0 {
        let headers: Vec<&str> = Vec::new();
        send_response(&mut stream,response::response_codes::HTTP_400_BAD_REQUEST,&headers,"");
        println!("Bad request");
        return Ok(());
    }

    // for (key, value) in &headers {
    //     if value.len() == 0 {
    //         println!("{}",key);
    //     } else {
    //         println!("{}:{}",key,value);
    //     }
    // }

    let path = match get_path(&headers) {
        Ok(str) => str,
        Err(v) => v
    };

    println!("{}",path);

    response::send_static_file(&mut stream, path)?;

    println!("Request processed\n");
    Ok(())
}
