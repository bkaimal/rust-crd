use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;
use futures::{StreamExt, TryStreamExt};
use kube::{
    api::{Api, ListParams},
    Client, CustomResource,
    runtime::{watcher, WatchStreamExt}
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
    let mut apply_stream = watcher(books, ListParams::default()).applied_objects().boxed();
    while let Some(event) = apply_stream.try_next().await? {
        println!("saw apply to {}", event.spec.title);
    }
    Ok(())
}