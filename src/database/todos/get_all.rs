use sqlx::{Pool, Error};
use sqlx_mysql::MySql;
use tokio::runtime::Runtime;
use crate::types::todo::ToDo;

async fn exec_async(pool: &Pool<MySql>) -> Result<Vec<ToDo>, Error> {
    let res = sqlx::query_as::<_, ToDo>("SELECT * FROM todos").fetch_all(pool).await;
    match res {
        Ok(res) => Ok(res),
        Err(er) => {
            println!("Failed to create todo on database, {}", er);
            Err(er)
        }
    }
}

pub fn get_all(pool: &Pool<MySql>) -> Result<Vec<ToDo>, bool> {
    match Runtime::new().unwrap().block_on(exec_async(pool)) {
        Ok(res) => Ok(res),
        Err(_) => Err(false)
    }
}