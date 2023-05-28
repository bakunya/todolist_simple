use sqlx::{Pool, Error};
use sqlx_mysql::MySql;
use tokio::runtime::Runtime;
use crate::types::todo::ToDo;

async fn exec_async(todo: ToDo, pool: Pool<MySql>) -> Result<bool, Error> {
    let raw = format!(
        "INSERT INTO `todos` 
            (`id`, `task`, `completed`, `id_user`) 
        VALUES ('{}', '{}', '{}', '{}')",
        todo.id.unwrap(),
        todo.task.unwrap(),
        todo.completed.unwrap(),
        todo.id_user.unwrap_or_default()
    );
    let res = sqlx::query(&raw).execute(&pool).await;
    match res {
        Ok(_) => Ok(true),
        Err(er) => {
            println!("Failed to create todo on database, {}", er);
            Err(er)
        }
    }
}

pub fn create(todo: ToDo, pool: Pool<MySql>) -> Result<bool, bool> {
    match Runtime::new().unwrap().block_on(exec_async(todo, pool)) {
        Ok(_) => Ok(true),
        Err(_) => Err(false)
    }
}