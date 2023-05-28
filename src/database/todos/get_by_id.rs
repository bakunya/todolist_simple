use sqlx::{Pool, Error};
use sqlx_mysql::MySql;
use tokio::runtime::Runtime;
use crate::types::todo::ToDo;

async fn exec_async(id: String, pool: &Pool<MySql>) -> Result<Vec<ToDo>, Error> {
    let raw = format!(
        "SELECT * FROM todos
        WHERE id = {};",
        id
    );

    let res = sqlx::query_as::<_, ToDo>(&raw).fetch_all(pool).await;
    match res {
        Ok(res) => Ok(res),
        Err(er) => {
            println!("Failed to create todo on database, {}", er);
            Err(er)
        }
    }
}

pub fn get_by_id(id: String, pool: &Pool<MySql>) -> Result<Vec<ToDo>, bool> {
    match Runtime::new().unwrap().block_on(exec_async(id, pool)) {
        Ok(res) => Ok(res),
        Err(_) => Err(false)
    }
}