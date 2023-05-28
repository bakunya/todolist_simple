use std::{net::TcpStream, io::Write};
use sqlx::Pool;
use sqlx_mysql::MySql;

use crate::{
    types::{
        http::HttpObject, 
        todo::ToDo
    }, 
    database::todos::update_by_id::update_by_id as update_todo,
};

pub fn handle_put(mut stream: TcpStream, http_object: HttpObject, pool: Pool<MySql>) {
    let id = http_object.get_query_string("id=").expect("id is invalid");
    let data_todo_vec: ToDo = serde_json::from_str(&http_object.body.unwrap()).unwrap();
    
    match update_todo(id, data_todo_vec, pool) {
        Ok(_) => {
            let response = format!("HTTP/1.1 201 OK\r\n");
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        },
        Err(_) => {
            let response = format!("HTTP/1.1 500 Server Error\r\n");
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}