use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;
use futures::{StreamExt, TryStreamExt};
use kube::{
    api::{Api, ListParams, WatchEvent},
    Client, CustomResource
};

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, Validate, JsonSchema)]
#[kube(group = "example.kaimal.net", version = "v1", kind = "Book", namespaced)]
pub struct BookSpec {
    pub title: String,
    pub authors: Option<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new client
    let client = Client::try_default().await?;

    // Watch for changes to books in the configured namespace
    let books: Api<Book> = Api::default_namespaced(client.clone());
    let mut stream = books.watch(&ListParams::default(), "0").await?.boxed();
    while let Some(event) = stream.try_next().await? {
        match event {
            WatchEvent::Added(book) => println!("ADDED: {} with name : {}", book.metadata.name.as_ref().unwrap().as_str(), book.spec.title),
            WatchEvent::Modified(book) => println!("UPDATED: {} with name : {}", book.metadata.name.as_ref().unwrap().as_str(), book.spec.title),
            WatchEvent::Deleted(book) => println!("DELETED: {}", book.metadata.name.as_ref().unwrap().as_str()),
            WatchEvent::Error(e) => println!("ERROR: {} {} ({})", e.code, e.message, e.status),
            _ => {}
        };
    }
    Ok(())
}