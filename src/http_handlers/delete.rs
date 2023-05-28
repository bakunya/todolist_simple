use std::{net::TcpStream, io::Write};
use sqlx::Pool;
use sqlx_mysql::MySql;

use crate::{
    types::{
        http::HttpObject, 
        todo::ToDo
    }, 
    database::todos::delete_by_id::delete_by_id as delete_todo,
};

pub fn handle_delete(mut stream: TcpStream, http_object: HttpObject, pool: Pool<MySql>) {
    let id = http_object.get_query_string("id=").expect("id is invalid");

    match delete_todo(id, &pool) {
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