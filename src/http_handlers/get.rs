use std::{net::TcpStream, io::Write};
use crate::{
    types::http::HttpObject, 
    utils::read_file::read as read_file
};

pub fn handle_get(mut stream: TcpStream, _: HttpObject) {
    let data_file: String = read_file();
    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}", data_file.len(), data_file);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}