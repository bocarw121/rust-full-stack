use std::vec;

use mongodb::{options::ClientOptions, Client, Collection};
use rocket::futures::TryStreamExt;
use types::Team;

use crate::{nba::get_teams, utils};

const TEAM_COLLECTION: &str = "teams";

pub struct Model {}

impl Model {
    // implement  favorite_team

    pub async fn get_all_teams() -> Vec<Team> {
        let collection = Self::get_collection(TEAM_COLLECTION.to_string()).await;

        // find all teams  if not just return and empty vec
        let cursor = match collection.find(None, None).await {
            Ok(cursor) => cursor,
            Err(_) => panic!("Error getting teams"),
        };

        cursor.try_collect().await.unwrap_or_else(|_| vec![])
    }

    // Implement a get_one_team
    pub async fn get_one_team(name: String) -> Team {
        let cursor = match Self::get_collection(TEAM_COLLECTION.to_string())
            .await
            .find(None, None)
            .await
        {
            Ok(cursor) => cursor,
            Err(_) => panic!("Error getting team"),
        };

        let team = match cursor
            .try_collect()
            .await
            .unwrap_or_else(|_| vec![])
            .into_iter()
            .find(|team| team.name.to_lowercase() == name)
        {
            Some(team) => team,
            None => Team::default(),
        };

        team
    }

    pub async fn get_collection(collection_name: String) -> Collection<Team> {
        let uri = utils::get_app_vars("MONGO_URI".to_owned());

        let mut client_options = match ClientOptions::parse(uri).await {
            Ok(client_options) => client_options,
            Err(e) => panic!("Error with uri {}", e),
        };

        // Manually set an option.
        client_options.app_name = Some("Test App".to_owned());

        let client = match Client::with_options(client_options) {
            Ok(client) => client,
            Err(e) => panic!("Error creating client: {}", e),
        };

        let db = client.database("nba-app");

        let collection = db.collection::<Team>(&collection_name);

        collection
    }

    pub async fn seed() {
        let collection = Self::get_collection("teams".to_owned()).await;

        let teams = get_teams();

        let result = collection.insert_many(teams, None).await;

        match result {
            Ok(result) => println!("Inserted {} documents", result.inserted_ids.len()),
            Err(e) => println!("Error inserting documents: {}", e),
        }
    }

    pub async fn get_length_of_items() -> u32 {
        let collection = Self::get_collection("teams".to_owned()).await;

        // find all teams  if not just return and empty vec
        let cursor = match collection.find(None, None).await {
            Ok(cursor) => cursor,
            Err(_) => panic!("Error finding teams"),
        };

        // collect the cursor into a vector
        let teams = cursor.try_collect().await.unwrap_or_else(|_| vec![]);

        teams.len() as u32
    }
}
