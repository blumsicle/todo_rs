#[macro_use]
extern crate log;

use anyhow::Result;
use chrono::{DateTime, Local};
use db::DB;
use std::env;

mod db;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let user = env::var("MONGODB_USER").unwrap();
    let pass = env::var("MONGODB_PASS").unwrap();
    let url = env::var("MONGODB_URL").unwrap();

    let db = DB::new(&user, &pass, &url).await?;

    // db.create_todo("another one").await?;

    let todos = db.fetch_todos().await?;
    for todo in todos {
        let local: DateTime<Local> = DateTime::from(todo.added_at);
        println!("todo: {}", todo.entry);
        println!("added at: {}", local);
    }

    Ok(())
}
