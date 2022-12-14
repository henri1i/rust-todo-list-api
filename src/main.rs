mod models;
mod repositories;
mod router;
mod http;
mod controllers;

use sqlx::{Pool, mysql::MySqlPoolOptions, MySql};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = set_up_pool().await.unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    http::serve(pool, addr).await?;

    Ok(())
}

async fn set_up_pool() -> Result<Pool<MySql>, sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&dotenvy::var("DATABASE_URL").unwrap()).await?;

    Ok(pool)
}
