use sqlx::{Pool, Error};
use sqlx_mysql::{MySql, MySqlQueryResult};
use tokio::runtime::Runtime;

async fn exec_async(id: String, pool: &Pool<MySql>) -> Result<MySqlQueryResult, Error> {
    let raw = format!(
        "DELETE FROM todos WHERE `id` = '{}'",
        id
    );
    sqlx::query(&raw).execute(pool).await
}

pub fn delete_by_id(id: String, pool: &Pool<MySql>) -> Result<bool, bool> {
    match Runtime::new().unwrap().block_on(exec_async(id, pool)) {
        Ok(_) => Ok(true),
        Err(er) => {
            println!("Error while query, {}", er);
            return Err(false);
        }
    }
}