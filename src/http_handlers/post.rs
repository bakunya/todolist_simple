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

pub fn handle_post(mut stream: TcpStream, http_object: HttpObject) {
    let todo_http: ToDo = serde_json::from_str(http_object.body.unwrap().as_str()).expect("fail");
    
    let mut todo = ToDo::init();
    todo.task = todo_http.task;
    todo.completed = Some(false);
    
    let data_vec: Option<Vec<ToDo>>;
    let data_file: String = read_file();
    
    if data_file != "" {
        let mut tmp: Vec<ToDo> = serde_json::from_str(data_file.as_str()).unwrap();
        tmp.push(todo);
        data_vec = Some(tmp);
    } else {
        data_vec = Some(vec![todo]);
    }

    let json = serde_json::to_string(&data_vec.unwrap()).unwrap();
    write_file(json);
    
    let response = format!("HTTP/1.1 201 OK\r\n");
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}