use std::net::{Shutdown};
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::prelude::*;
use crate::headers::get_path;
use crate::response::response_actions::*;
use crate::response::response_codes::HTTP_400_BAD_REQUEST;

mod headers;
mod helpers;
mod response;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Enable stacktrace
    std::env::set_var("RUST_BACKTRACE", "1"); 
 
    // Ip and port to run on
    let address = "127.0.0.1:4200";

    // Start the server
    let result = run_server(address).await;

    if result.is_err() {        
        print!("Server exited with error: \n{}", result.unwrap_err());
    } else {
        print!("Done")
    }
        
    Ok(())
}
 
async fn run_server(address: &str) -> std::io::Result<()> {
    // Bind to the given address to listen to traffic
    let listener_result = TcpListener::bind(address).await?;
    
    println!("Server listening on {}", address);

    // Listen for incoming traffic
    let mut incoming = listener_result.incoming();
    while let Some(stream) = incoming.next().await{
        println!("Incoming request");
        // Process request
        process_client_request(stream.unwrap()).await?;
        println!("Waiting for request");
        println!();
    }
    Ok(())
}

async fn process_client_request(mut stream: TcpStream) -> std::io::Result<()> {    
    let (headers,body) = match headers::read_request(&mut stream).await {
        Ok(result) => result,
        Err(_) => {
            stream.shutdown(Shutdown::Both)?;
            panic!("Failed to read request")
        }
    };   


    if headers.len() == 0 {
        let headers: Vec<&str> = Vec::new();
        send_response(&mut stream,HTTP_400_BAD_REQUEST,&headers,"").await;
        println!("Bad request");
        return Ok(());
    }
   
   
    for header in &headers {        
        print!("{}:",&header.0);
        print!("{}",&header.1);
        println!();
        
    }

    let path = match get_path(&headers) {
        Ok(str) => str,
        Err(v) => v
    };
    
    send_static_file(&mut stream, path, &body).await?;
    println!("Request processed\n");
    Ok(())
}
