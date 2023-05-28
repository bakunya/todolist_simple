use std::{net::TcpStream, io::Write};
use sqlx::Pool;
use sqlx_mysql::MySql;

use crate::{
    types::http::HttpObject,
    database::todos::{get_all::get_all as get_todos, get_by_id::get_by_id as get_todo}
};

pub fn handle_get(mut stream: TcpStream, http_object: HttpObject, pool: Pool<MySql>) {
    let id = http_object.get_query_string("id=");

    if !id.is_none() {
        return match get_todo(id.expect("ID is invalid"), &pool) {
            Ok(res) => {
                let json = serde_json::to_string(&res).unwrap();
                let response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}", json.len(), json);
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

    return match get_todos(&pool) {
        Ok(res) => {
            let json = serde_json::to_string(&res).unwrap();
            let response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}", json.len(), json);
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