pub mod client;
pub mod collections;
pub mod db;
pub mod seed;
pub mod utils;

use std::vec;

use rocket::{futures::{TryStreamExt, StreamExt}, http::{Status, ext::IntoCollection}, serde::json::Json};
use serde::{Deserialize, Serialize};
use types::{FavTeam, FavTeamPayload, Team};

use crate::nba::get_teams;

use self::collections::{fav_team_collection, team_collection};
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

    pub async fn add_favorite_team(payload: Json<FavTeamPayload>) -> Message {
        let collection = fav_team_collection().await;
        let teams = get_teams();
        
        // iterate through array of teams to get the team data
        let teams = teams.iter().find_map(|team| {
            if team.name.to_lowercase() == payload.team_name.to_lowercase() {
                Some(team)
            } else {
                None
            }
        });

        let team = match teams {
            Some(team) => team,
            None => return Message {
                status: Status::NotFound.to_string(),
                message: "Please enter a valid team name".to_owned(),
            },
        };

       
        let fav_team = FavTeam {
            user_name: payload.user_name.to_owned(),
            team: team.clone(),
        };

        let status = match collection.insert_one(fav_team.clone(), None).await {
            Ok(_) => Message {
                status: Status::Created.to_string(),
                message: "Favorite team created Successfully".to_owned(),
            },
            Err(_) => Message {
                status: Status::Conflict.to_string(),
                message: "Something went wrong, please try again".to_owned(),
            },
        };

        status
    }

    pub async fn get_all_favorite_teams(user_name: String) -> Vec<FavTeam> {
        let collection = fav_team_collection().await;

        let cursor = match collection.find(None, None).await {
            Ok(cursor) => cursor,
            Err(_) => panic!("Error getting teams"),
        };
  
        // Filter out the favorite team for a specific user.
       let fav_team = cursor.try_collect().await.unwrap_or_else(|_| vec![]).into_iter().filter(|favteam| favteam.user_name.to_lowercase() == user_name.to_owned()).collect();

       fav_team
    }
}
