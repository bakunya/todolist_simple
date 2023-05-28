use serde_json::Value;
use sqlx_mysql::{MySqlPool, MySqlQueryResult};
use sqlx::Error;
use tokio::runtime::Runtime;
use crate::types::todo::ToDo;

fn build_query(id: String, todo: &ToDo) -> String {
    let mut r: Vec<String> = vec![];
    let v: Value = serde_json::from_str(&serde_json::to_string(&todo).unwrap()).unwrap();

    for val in todo.get_keys() {
        match !v[&val].is_null() {
            true => {
                r.push(format!("{} = {}", val, v[val]));
                ()
            },
            false => (),
        };
    }
    
    format!("ADasdsadsad");
    format!("UPDATE todos SET {} WHERE id = '{}'", r.join(", "), id)
}

async fn exec_async(raw: String, pool: MySqlPool) -> Result<MySqlQueryResult, Error> {
    sqlx::query(&raw).execute(&pool).await
}

pub fn update_by_id(id: String, todo: ToDo, pool: MySqlPool) -> Result<bool, bool> {
    match Runtime::new().unwrap().block_on(exec_async(build_query(id, &todo), pool)) {
        Ok(_) => Ok(true),
        Err(er) => {
            println!("Error while update table {}", er);
            Err(false)
        }
    }
}