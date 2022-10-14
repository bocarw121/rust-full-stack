pub mod client;
pub mod collections;
pub mod db;
pub mod utils;

use std::vec;

use mongodb::{
    bson::{self, doc},
    options::{ UpdateOptions},
};
use rocket::{
    futures::{ TryStreamExt},
    serde::json::Json,
};
use serde::{Deserialize, Serialize};
use types::{FavTeam, FavTeamPayload, NBATeams, Team};

use crate::{
    nba::get_teams,
    responses::{error::{Error, ErrorResponse}, success::{SuccessResponsesNoData, Success}},
};

use self::collections::{fav_team_collection, team_collection};
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Message {
    status: String,
    message: String,
}

pub struct Model();

// Holds all functions for the database calls
impl Model {
    pub async fn generate_nba_teams(id: String) {
        let collection = team_collection().await;

        let teams = NBATeams {
            _id: id.clone(),
            teams: get_teams(id),
        };

        let result = collection.insert_one(teams, None).await;

        match result {
            Ok(result) => println!("Inserted {} documents", result.inserted_id),
            Err(e) => println!("Error inserting documents: {}", e),
        }
    }

    pub async fn get_all_teams(id: String) -> Result<Vec<NBATeams>, String> {
        let collection = team_collection().await;

        let cursor = match collection.find(doc! {"_id": id}, None).await {
            Ok(cursor) => cursor,
            Err(_) => return Err("Unable to get all teams".to_string()),
        };

        Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
    }

    pub async fn get_one_team(name: String, user_id: String) -> Result<Team, String> {
        let doc = doc! {"_id": user_id};
        let cursor = match team_collection().await.find(doc, None).await {
            Ok(cursor) => cursor,
            Err(_) => return Err("User not found".to_string()),
        };
        let team_iter = cursor
            .try_collect()
            .await
            .unwrap_or_else(|_| vec![])
            .into_iter();

        let team_vec = team_iter
            .into_iter()
            .flat_map(|teams| teams.teams.into_iter())
            .collect::<Vec<Team>>();

        let team = match team_vec
            .into_iter()
            .find(|team| team.name.to_lowercase() == name.to_lowercase())
        {
            Some(team) => team,
            None => return Err("Invalid request, check user id or team".to_string()),
        };

        Ok(team)
    }

    pub async fn add_favorite_team(payload: Json<FavTeamPayload>) -> Result<Success, Error> {
        let collection = fav_team_collection().await;

        // Use the get one team model to get the teams
        let team =
            match Self::get_one_team(payload.team_name.clone(), payload.user_id.clone()).await {
                Ok(team) => team,
                Err(e) => return Err(ErrorResponse::create_400_error(&e.to_string()))
            };

        // Make a bson for type team to pass to docs
        let team_bson = match bson::to_bson(&team) {
            Ok(team_bson) => team_bson,
            Err(_) => return Err(ErrorResponse::create_500_error(&"Something went wrong, Please try again later".to_string()))
        };

        let docs = doc! {"_id": payload.user_id.clone(), };
        // No duplicates $addToSet upserts if the document doesn't exist

        let update = doc! {"$addToSet": {"team": team_bson}};

        //  build options and set upsert to true
        let options = UpdateOptions::builder().upsert(true).build();

        let status = match collection.update_one(docs, update, options.clone()).await {
            Ok(_) => Ok(SuccessResponsesNoData::created(format!("{} successfully added to favorites list", team.name))),
            Err(_) => Err(ErrorResponse::create_500_error(&"Something went wrong, please try again".to_string())),
        };

 

        status
    }

    pub async fn get_all_favorite_teams(user_id: String) -> Vec<FavTeam> {
        let collection = fav_team_collection().await;

        let fav_teams = match collection.find(doc! {"_id": user_id}, None).await {
            Ok(cursor) => cursor,
            Err(_) => panic!("Error getting favorite teams"),
        };

        fav_teams.try_collect().await.unwrap_or_else(|_| vec![])
    }

    pub async fn delete_favorite_team(team_name: String, user_id: String) -> Result<Success, Error> {
        let collection = fav_team_collection().await;

        let doc = doc! {"_id": user_id.clone()};
        let cursor = match collection.find(doc, None).await {
            Ok(cursor) => cursor,
            Err(_) => return Err(ErrorResponse::create_400_error(&"Invalid user id".to_string())),
        };
        let fav_team_iter = cursor
            .try_collect()
            .await
            .unwrap_or_else(|_| vec![])
            .into_iter();

        let fav_team_vec = fav_team_iter
            .into_iter()
            .flat_map(|teams| teams.team.into_iter())
            .collect::<Vec<Team>>();

        let team = match fav_team_vec
            .into_iter()
            .find(|team| team.name.to_lowercase() == team_name.to_lowercase())
        {
            Some(team) => team,
            None => return  Err(ErrorResponse::create_400_error(&"Invalid user id or team name".to_string())),
        };

        // let team_update_key = format!("_id", team._id);

        let query = doc! {"_id": user_id };
        let update = doc! {"$pull": {"team": {"_id":team._id } }};

        let result = match collection.update_many(query, update, None).await {
            Ok(result) => result,
            Err(_) => return Err(ErrorResponse::create_400_error(&"Something went wrong, please try again")),
        };

        println!("{:?}", result);

        Ok(SuccessResponsesNoData::ok("Team successfully deleted".to_string()))
    }

    pub async fn update_one_team(user_name: String, user_id: String) -> Result<Success, Error> {
        let team = Self::get_one_team(user_name.clone(), user_id.clone()).await;

        let team = match team {
            Ok(team) => team,
            Err(_) => return Err(ErrorResponse::create_400_error(&"Invalid user id".to_string()))
            
        };

        let collection = team_collection().await;

        // This will access the correct index for the nba team that needs to be updated
        let team_update_key = format!("teams.{}.is_favorite", team._id - 1);

        let result = match collection
            .update_one(
                doc! {"_id": user_id},
                doc! {"$set": {team_update_key:!team.is_favorite}},
                None,
            )
            .await
        {
            Ok(result) => result,
            Err(_) => return Err(ErrorResponse::create_500_error(&"Something went wrong "))
        };

        let message = if team.is_favorite == true {
            format!("You successfully added {} to favorites", team.name)
        } else {
            format!("You successfully removed {} from favorites", team.name)
        };

       Ok(SuccessResponsesNoData::updated(message.to_string()))
    }
}
