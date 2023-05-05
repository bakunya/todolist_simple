use std::{net::TcpStream, io::Write};
use crate::{
    types::{
        http::HttpObject, 
        todo::ToDo
    }, 
    utils::{
        read_file::read as read_file, 
        write_file::write as write_file
    }
};

pub fn handle_delete(mut stream: TcpStream, http_object: HttpObject) {
    let data_file = read_file();
    let id = http_object.query_string.unwrap().split("&").nth(0).unwrap().to_string().replace("id=", "").trim().to_string();

    let mut data_vec: Vec<ToDo> = serde_json::from_str(&data_file).unwrap();
    data_vec.retain(|v| v.clone().id.unwrap() != id);
    
    let json = serde_json::to_string(&data_vec).unwrap();
    write_file(json);

    let response = format!("HTTP/1.1 201 OK\r\n");
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}