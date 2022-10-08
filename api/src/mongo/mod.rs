pub mod client;
pub mod collections;
pub mod db;
pub mod seed;
pub mod utils;

use std::vec;

use rocket::{futures::TryStreamExt, http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use types::{FavTeam, FavTeamWithId, Team};

use self::collections::{fav_team_collection, team_collection};
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ErrorMessage {
    status: String,
    message: String,
}

pub struct Model();

impl Model {
    pub async fn get_all_teams() -> Vec<Team> {
        let collection = team_collection().await;

        // find all teams  if not just return and empty vec
        let cursor = match collection.find(None, None).await {
            Ok(cursor) => cursor,
            Err(_) => panic!("Error getting teams"),
        };

        cursor.try_collect().await.unwrap_or_else(|_| vec![])
    }

    // Implement a get_one_team
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

    // implement  favorite_team
    pub async fn add_favorite_team(payload: Json<FavTeam>) -> ErrorMessage {
        let collection = fav_team_collection().await;

        let fav_team = FavTeam {
            user_name: payload.user_name.to_owned(),
            team_name: payload.team_name.to_owned(),
            city: payload.city.to_owned(),
        };

        // combines the users favorite team with the struct that has a rand _id
        let fav_team_with_id = FavTeamWithId {
            _id: rand::random(),
            fav_team,
        };

        let status = match collection.insert_one(fav_team_with_id, None).await {
            Ok(_) => ErrorMessage {
                status: Status::Created.to_string(),
                message: "Favorite team created Successfully".to_owned(),
            },
            Err(e) => ErrorMessage {
                status: Status::Conflict.to_string(),
                message: e.to_string(),
            },
        };

        status
    }

    pub async fn get_favorite_team() -> Vec<FavTeamWithId> {
        let collection = fav_team_collection().await;

        // find all teams  if not just return and empty vec
        let cursor = match collection.find(None, None).await {
            Ok(cursor) => cursor,
            Err(_) => panic!("Error getting teams"),
        };

        cursor.try_collect().await.unwrap_or_else(|_| vec![])
    }
}
