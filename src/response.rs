use std::collections::BTreeMap;
use async_std::io::WriteExt;
use async_std::net::TcpStream;
use crate::helpers::{write_string, get_extension};
use chrono::Utc;
use std::io::Read;
use std::io::BufReader;

pub mod response_codes {
    pub const HTTP_200_OK: &str = "200 OK";

    pub const HTTP_404_NOT_FOUND: &str = "404 Not Found";

    pub const HTTP_400_BAD_REQUEST: &str = "400 Bad Request";
}


pub async fn send_static_file(stream: &mut TcpStream, requested_path: &str, request_body: &str) -> std::io::Result<()> {
    let mut path = String::from("wwwroot/");

    let mut end_index = requested_path.len();    

    match requested_path.chars().position(|c| c == '?') {
        Some(v) => end_index = v,
        None => {},
    };

    match requested_path.chars().position(|c| c == '#') {
        Some(v) => {
            if v < end_index {
                end_index = v;
            }
        },
        None => {},
    }
    


    if end_index > 0 && requested_path.len() > 1 {
        path.push_str(&requested_path[1..end_index]);
    } else {
        path.push_str("index.html");
    }

    let status_code: &str;
        
    let mut headers: Vec<&str> = Vec::new();
    println!("path {}",path.as_str());
    let body: String;
    let file_exists = std::path::Path::new(path.as_str()).is_file();
    match std::fs::read_to_string(path.as_str()) {
        Ok(v) => {
            body = render_page(&v, &headers, request_body);
            status_code = response_codes::HTTP_200_OK;
            headers.push(get_content_type_header(path.as_str()));
        }
        Err(_) => {
            if file_exists {
                send_binary_file(stream, path.as_str()).await;
                return Ok(());
            } else {                
                body = "<html><body><h1>Not found</h1><p>Page not found</p></body></html>".to_owned();
                status_code = response_codes::HTTP_404_NOT_FOUND;
                headers.push("content-type: text/html");
            }            
        }
    };

    send_response(stream,status_code,&headers,body.as_str()).await;
    Ok(())
}

async fn send_binary_file(stream: &mut TcpStream, path: &str) {
    let content_type = get_content_type_header(path);
    let mut headers : Vec<&str> = Vec::new();
    headers.push(content_type);

    let file = std::fs::File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();

    let content_length = std::format!("content-length: {}", buffer.len());    
    headers.push(content_length.as_str());

    write_headers(stream, response_codes::HTTP_200_OK, &headers).await;

  
    match stream.write_all(&buffer).await {
        Ok(_) => (),
        Err(e) => {
            println!("Error when writing binary file {}", e);
        }
    };
}


pub async fn send_response(stream: &mut TcpStream, status_code: &str, additional_headers: &Vec<&str>, body: &str) {
    write_headers(stream, status_code, additional_headers).await;

    if body.len() > 0 {
        write_string(stream, body).await;
    }
}

async fn write_headers(stream: &mut TcpStream, status_code: &str, additional_headers: &Vec<&str>) {
    // Http Status code
    write_string(stream, "HTTP/1.1 ").await;
    write_string(stream, status_code).await;
    write_string(stream, "\n").await;

    println!("{}", status_code);
    // Send time
    let now = Utc::now();
    let formdatted_date = now.format("%a, %d %b %Y %T GMT").to_string();
    let date_header = std::format!("date: {}\n",formdatted_date);
    
    write_string(stream, date_header.as_str()).await;
    // Server name
    write_string(stream, "Server: RnD Poopy Rust Server\n").await;
    // Additional headers
    if additional_headers.len() > 0 {
        for header in additional_headers {
            write_string(stream, header).await;
            write_string(stream, "\n").await;
        }
    }
    write_string(stream, "\n").await;
}

// Get content type header based on file extension
fn get_content_type_header(file: &str) -> &str {
    let extension: &str;
    match get_extension(file) {
        Ok(ex) => extension = ex,
        Err(_) => return "content-type:octet-stream"
    }
    
    if extension == ".html" {
        return "content-type:text/html; charset=utf-8";
    }
    else if extension == ".png" { 
        return "content-type:image/png";
    }
    else if extension == ".jpg" {
        return "content-type:image/jpeg";
    }
    else if extension == ".txt" {
        return "content-type:text/plain";
     }
     else {
        return "content-type:octet-stream";
    }
}

fn render_page(contents: &str, _headers: &Vec<&str>, body: &str) -> String {
    let mut data = BTreeMap::new();

    // TODO query params

    // TODO Multipart form
    let lines = body.split('\n');
    println!("body length {}",body.len());
    
    lines.for_each(|item| {
        let items = item.split('=').collect::<Vec<&str>>();
        if items.len() > 1 {
            data.insert(items[0], items[1]);
            println!("PostKey: {}",items[0]);
            println!("PostValue: {}",items[1]);
        }
    });

    return render_handlebars_template(contents, &data);
}

fn render_handlebars_template(template: &str, data: &BTreeMap<&str,&str>) -> String {
    let mut handlebars = handlebars::Handlebars::new();

    assert!(handlebars.register_template_string("t1", template).is_ok());

    return handlebars.render("t1", data).unwrap();
}