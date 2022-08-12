use anyhow::Result;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::{ClientOptions, FindOptions},
    Client, Cursor,
};
use serde::{Deserialize, Serialize};

const DB_NAME: &str = "todo_rs";
const COLLECTION: &str = "todo";

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    #[serde(rename = "_id")]
    #[serde(with = "mongodb::bson::serde_helpers::hex_string_as_object_id")]
    pub id: String,

    pub entry: String,

    #[serde(with = "mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub added_at: DateTime<Utc>,
}

pub struct DB {
    pub client: Client,
}

impl DB {
    pub async fn new(username: &str, password: &str, url: &str) -> Result<Self> {
        let uri = format!("mongodb+srv://{username}:{password}@{url}/?retryWrites=true&w=majority");

        let mut client_options = ClientOptions::parse(uri).await?;
        client_options.app_name = Some("todo_rs".to_string());

        let client = Client::with_options(client_options)?;
        client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await?;

        info!(target: "mongodb", "connected successfully");

        Ok(Self { client })
    }

    pub async fn create_todo(&self, entry: &str) -> Result<()> {
        let doc = doc! {
            "entry": entry.to_string(),
            "added_at": mongodb::bson::DateTime::from_chrono(Utc::now()),
        };

        self.client
            .database(DB_NAME)
            .collection(COLLECTION)
            .insert_one(doc, None)
            .await?;

        info!(target: "mongodb", "todo created");

        Ok(())
    }

    pub async fn delete_todo(&self, id: &str) -> Result<()> {
        let oid = ObjectId::parse_str(id)?;

        self.client
            .database(DB_NAME)
            .collection::<Todo>(COLLECTION)
            .delete_one(doc! {"_id": oid}, None)
            .await?;

        info!(target: "mongodb", "todo deleted");

        Ok(())
    }

    pub async fn fetch_todos(&self) -> Result<Vec<Todo>> {
        let options = FindOptions::builder().sort(doc! { "added_at": 1 }).build();

        let mut cursor: Cursor<Todo> = self
            .client
            .database(DB_NAME)
            .collection(COLLECTION)
            .find(None, options)
            .await?;

        info!(target: "mongodb", "todos fetched");

        let mut result: Vec<Todo> = Vec::new();
        while let Some(doc) = cursor.try_next().await? {
            result.push(doc.into());
        }

        Ok(result)
    }
}
