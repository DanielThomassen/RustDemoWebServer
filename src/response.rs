use std::net::TcpStream;
use crate::helpers::write_string;

pub fn send_html_page(stream: &mut TcpStream, requested_path: &str) -> std::io::Result<()> {
    let mut path = String::from("wwwroot/");

    if requested_path.len() > 1 {
        path.push_str(&requested_path[1..]);
    } else {
        path.push_str("index.html");
    }

    let status_code: &str;
    let body: String;
    match std::fs::read_to_string(path) {
        Ok(v) => {
            body = v;
            status_code = "200 OK";
        }
        Err(_) => {
            body = "<html><body><h1>Not found</h1><p>Page not found</p></body></html>".to_owned();
            status_code = "404 Not Found"
        }
    };
    send_response(stream,status_code,"content-type: text/html",body.as_str());
    Ok(())
}


pub fn send_response(stream: &mut TcpStream, status_code: &str, additional_headers: &str, body: &str) {
    write_string(stream, "HTTP/1.1 ");
    write_string(stream, status_code);
    write_string(stream, "\n");
    if additional_headers.len() > 0 {
        write_string(stream, additional_headers);
    }
    write_string(stream, "\n\n");

    if body.len() > 0 {
        write_string(stream, body);
    }
}