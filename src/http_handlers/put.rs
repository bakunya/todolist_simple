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

pub fn handle_put(mut stream: TcpStream, http_object: HttpObject) {
    let data_file = read_file();
    let id = http_object.query_string.unwrap().split("&").nth(0).unwrap().to_string().replace("id=", "").trim().to_string();

    let data_todo_vec: ToDo = serde_json::from_str(&http_object.body.unwrap()).unwrap();
    let mut data_vec: Vec<ToDo> = serde_json::from_str(&data_file).unwrap();

    let mut i = 0;
    
    loop {
        if i >= data_vec.iter().count() {
            break;
        }

        let curr = data_vec.get_mut(i).unwrap();

        if curr.id.as_ref().unwrap() == &id {
            if data_todo_vec.completed != None {
                curr.completed = data_todo_vec.clone().completed;
            }
            
            if data_todo_vec.task != None {
                curr.task = data_todo_vec.clone().task;
            }

            break;
        }

        i += 1;
    }
    
    let json = serde_json::to_string(&data_vec).unwrap();
    write_file(json);

    let response = format!("HTTP/1.1 201 OK\r\n");
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}