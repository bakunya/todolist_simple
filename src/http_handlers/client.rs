use std::{net::TcpStream, io::{Read, Write}, fs::File};
use sqlx::{Pool};
use sqlx_mysql::{MySql};
use crate::types::http::HttpObject;

use super::{get::handle_get, put::handle_put, post::handle_post, delete::handle_delete, error::handle_error};

fn handle_json(stream: TcpStream, http_object: HttpObject, pool: Pool<MySql>) {    
    match &http_object.method {
        Some(val) => {
            match val.as_str() {
                "GET" => {
                    return handle_get(stream, http_object, pool);
                },
                "PUT" => {
                    return handle_put(stream, http_object, pool);
                },
                "POST" => {
                    return handle_post(stream, http_object, pool);
                },
                "DELETE" => {
                    return handle_delete(stream, http_object, pool)
                },
                _ => {
                    return handle_error(stream)
                }
            }
        },
        None => {}
    }
}

fn handle_file(mut stream: TcpStream, http_object: HttpObject) {
    if !http_object.file_valid.unwrap() {
        stream.write_all("HTTP/1.1 401 Unauthorized\r\n\r\n".to_string().as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    }

    match File::open(format!("./src/static/{}", http_object.file.unwrap())) {
        Ok(mut file) => {
            let mut contents = Vec::new();
            file.read_to_end(&mut contents).unwrap();
            let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", contents.len(), String::from_utf8_lossy(&contents));
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        },
        Err(_) => {
            stream.write_all("HTTP/1.1 404 Not Found\r\n\r\nFile not found".to_string().as_bytes()).unwrap();
            stream.flush().unwrap();
        },
    }
}


pub fn handle_client(mut stream: TcpStream, pool: Pool<MySql>) {
    let mut buffer = [0; 2048];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let request_object = request.split("\r\n").collect::<Vec<_>>();
    
    let mut http_object = HttpObject::init();
    http_object.to_object(request_object);

    match http_object.get_http_header("content-type").ok_or("content type not found") {
        Ok(val) => {
            match val.to_lowercase() == "application/json" {
                true => handle_json(stream, http_object, pool),
                false => handle_file(stream, http_object)
            }
        },
        Err(_) => handle_file(stream, http_object)
    }
}