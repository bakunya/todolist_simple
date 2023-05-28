use std::{net::TcpStream, io::Write};
use sqlx::Pool;
use sqlx_mysql::MySql;

use crate::{
    types::{
        http::HttpObject, 
        todo::ToDo
    }, 
    database::{
        todos::create::create as create_todo
    }
};

pub fn handle_post(mut stream: TcpStream, http_object: HttpObject, pool: Pool<MySql>) {
    let todo_http: ToDo = serde_json::from_str(http_object.body.unwrap().as_str()).expect("fail");
    
    let mut todo = ToDo::init();
    todo.task = todo_http.task;
    todo.completed = Some(false);

    match create_todo(todo, pool) {
        Ok(_) => {
            let response = format!("HTTP/1.1 201 OK\r\n");
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        },
        Err(_) => {
            let response = format!("HTTP/1.1 500 SERVER ERROR\r\n");
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}