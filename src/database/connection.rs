use sqlx::Pool;
use sqlx_mysql::{MySql};

pub async fn connect() -> Result<Pool<MySql>, sqlx::Error> {
    let pool = Pool::<MySql>::connect("mysql://root@localhost:3306/todos").await?;
    Ok(pool)
}
