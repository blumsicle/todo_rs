#[macro_use]
extern crate log;

use anyhow::Result;
use chrono::{DateTime, Local};
use clap::{Parser, Subcommand};
use db::DB;
use std::env;

mod db;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { entry: String },
    Delete { num: usize },
    List,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let cli = Cli::parse();

    let user = env::var("MONGODB_USER").unwrap();
    let pass = env::var("MONGODB_PASS").unwrap();
    let url = env::var("MONGODB_URL").unwrap();

    let db = DB::new(&user, &pass, &url).await?;

    match &cli.command {
        Commands::Add { entry } => {
            db.create_todo(&entry).await?;
        }

        Commands::Delete { num } => {
            let todos = db.fetch_todos().await?;
            for (i, todo) in todos.iter().enumerate() {
                if i == *num {
                    db.delete_todo(&todo.id).await?;
                }
            }
        }

        Commands::List => {
            let todos = db.fetch_todos().await?;
            for (i, todo) in todos.iter().enumerate() {
                let local: DateTime<Local> = DateTime::from(todo.added_at);
                println!("[{i}] {} ({})", todo.entry, local);
            }
        }
    }

    Ok(())
}
