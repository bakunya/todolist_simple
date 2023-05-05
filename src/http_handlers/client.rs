use std::{net::TcpStream, io::Read};

use crate::{http_handlers::{get::handle_get, post::handle_post, delete::handle_delete, put::handle_put, error::handle_error}, types::http::HttpObject};

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let request_object = request.split("\r\n").collect::<Vec<_>>();
    
    let mut http_object = HttpObject::init();
    http_object.to_object(request_object);

    match &http_object.method {
        Some(val) => {
            match val.as_str() {
                "GET" => {
                    return handle_get(stream, http_object);
                },
                "PUT" => {
                    return handle_put(stream, http_object);
                },
                "POST" => {
                    return handle_post(stream, http_object);
                },
                "DELETE" => {
                    return handle_delete(stream, http_object)
                },
                _ => {
                    return handle_error(stream)
                }
            }
        },
        None => {}
    }
}