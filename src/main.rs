use std::net::{Shutdown};
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::prelude::*;
use crate::headers::get_path;
use crate::response::send_response;
use futures::executor::block_on;

mod headers;
mod response;
mod helpers;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");
 
    let address = "127.0.0.1:4200";

    block_on(run_server(address));
        
    Ok(())
}
 
async fn run_server(address: &str) -> std::io::Result<()> {
    let listener_result = TcpListener::bind(address).await?;
    
    println!("Server listening on {}", address);

    let mut incoming = listener_result.incoming();
    while let Some(stream) = incoming.next().await{
        handle_client(stream.unwrap()).await?;
    }
    Ok(())
}

async fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {    
    let (headers,body) = match headers::read_request(&mut stream).await {
        Ok(result) => result,
        Err(_) => {
            stream.shutdown(Shutdown::Both)?;
            panic!("Failed to read request")
        }
    };
    
    if headers.len() == 0 {
        let headers: Vec<&str> = Vec::new();
        send_response(&mut stream,response::response_codes::HTTP_400_BAD_REQUEST,&headers,"").await;
        println!("Bad request");
        return Ok(());
    }

    let path = match get_path(&headers) {
        Ok(str) => str,
        Err(v) => v
    };
    println!("{}",path);


    response::send_static_file(&mut stream, path, &body).await?;
    println!("Request processed\n");
    Ok(())
}
