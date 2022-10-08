use crate::nba::get_teams;

use super::collections::team_collection;

pub async fn seed() {
    let collection = team_collection().await;

    let teams = get_teams();

    let result = collection.insert_many(teams, None).await;

    match result {
        Ok(result) => println!("Inserted {} documents", result.inserted_ids.len()),
        Err(e) => println!("Error inserting documents: {}", e),
    }
}
