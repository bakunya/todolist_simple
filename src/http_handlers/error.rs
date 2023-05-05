use std::{net::TcpStream, io::Write};


pub fn handle_error(mut stream: TcpStream) {
    let response = format!("HTTP/1.1 500 OK\r\n");
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}