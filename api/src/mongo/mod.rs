pub mod client;
pub mod collections;
pub mod db;
pub mod seed;
pub mod utils;

use std::vec;

use rocket::{futures::TryStreamExt, http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use types::{FavTeam,  Team};

use self::collections::{fav_team_collection, team_collection, };
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Message {
    status: String,
    message: String,
}

pub struct Model();

// Holds all functions for the database calls
impl Model {
    pub async fn get_all_teams() -> Vec<Team> {
        let collection = team_collection().await;

      
        let cursor = match collection.find(None, None).await {
            Ok(cursor) => cursor,
            Err(_) => panic!("Error getting teams"),
        };

        cursor.try_collect().await.unwrap_or_else(|_| vec![])
    }

 
    pub async fn get_one_team(name: String) -> Team {
        let cursor = match team_collection().await.find(None, None).await {
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


    pub async fn add_favorite_team(payload: Json<FavTeam>) -> Message {
        let collection = fav_team_collection().await;

        let fav_team = FavTeam {

            user_name: payload.user_name.to_owned(),
            team_name: payload.team_name.to_owned(),
            city: payload.city.to_owned(),
        };

    
        let status = match collection.insert_one(fav_team, None).await {
            Ok(_) => Message {
                status: Status::Created.to_string(),
                message: "Favorite team created Successfully".to_owned(),
            },
            Err(error) => Message {
                status: Status::Conflict.to_string(),
                message: error.to_string(),
            },
        };

        status
    }

    pub async fn get_all_favorite_teams() -> Vec<FavTeam> {
        let collection = fav_team_collection().await;

        let cursor = match collection.find(None, None).await {
            Ok(cursor) => cursor,
            Err(_) => panic!("Error getting teams"),
        };

        cursor.try_collect().await.unwrap_or_else(|_| vec![])

  
    }
}
