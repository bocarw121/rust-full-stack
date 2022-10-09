use mongodb::Database;

use super::client;

pub async fn create() -> Database {
    let client = client::client().await;
    let db = client.database("nba-app");
    db
}
