pub mod client;
pub mod collections;
pub mod db;
pub mod utils;

use std::vec;

use mongodb::bson::doc;
use rocket::{
    futures::{TryStreamExt, TryFutureExt},
    http::{ Status, ext::IntoCollection},
    serde::json::Json,
};
use serde::{Deserialize, Serialize};
use types::{FavTeam, FavTeamPayload, Team, NBATeams};

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


    pub async fn get_all_teams(id: String) -> Vec<NBATeams> {
        let collection = team_collection().await;

        let cursor = match collection.find(doc! {"_id": id} , None).await {
            Ok(cursor) => cursor,
            Err(_) => panic!("Error getting teams"),
        };

        cursor.try_collect().await.unwrap_or_else(|_| vec![])
    }

    pub async fn get_one_team(name: String) -> Team  {
           let cursor = match team_collection().await.find(None, None).await {
            Ok(cursor) => cursor,
            Err(_) => panic!("Error getting team"),
        };

        let team_iter = cursor
            .try_collect()
            .await
            .unwrap_or_else(|_| vec![])
            .into_iter();

        let team_vec = team_iter.into_iter().flat_map(|teams| teams.teams.into_iter()).collect::<Vec<Team>>();
        
        let team = match team_vec.into_iter().find(|team| team.name.to_lowercase() == name.to_lowercase())  {
            Some(team) => team,
            None => panic!("No team found")
        }; 
   
     team


    }
// TODO
    pub async fn add_favorite_team(payload: Json<FavTeamPayload>) -> Message {
        let collection = fav_team_collection().await;

        // Use the get one team model to get the team
        let team = Self::get_one_team(payload.team_name.clone()).await;

        let updated_team = Team {
            _id: team._id,
            name: team.name,
            city: team.city,
            logo: team.logo,
            is_favorite: true,
        };

        let fav_team = FavTeam {
            _id: payload.user_id.to_owned(),
            team: updated_team,
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
        let fav_team = cursor
            .try_collect()
            .await
            .unwrap_or_else(|_| vec![])
            .into_iter()
            .filter(|favteam| favteam._id.to_lowercase() == user_name.to_owned())
            .collect();

        fav_team
    }
}

































//  pub async fn add_favorite_team(payload: Json<FavTeamPayload>) -> Message {
//         let collection = fav_team_collection().await;

//         // make request do db for teams here
//         let teams = get_teams("".to_owned());

//         // iterate through array of teams to get the team data
//         let teams = teams.iter().find_map(|team| {
//             if team.name.to_lowercase() == payload.team_name.to_lowercase() {
//                 Some(team)
//             } else {
//                 None
//             }
//         });

//         let team = match teams {
//             Some(team) => team,
//             None => {
//                 return Message {
//                     status: Status::NotFound.to_string(),
//                     message: "Please enter a valid team name".to_owned(),
//                 }
//             }
//         };

//         let team = team.clone();

//         let updated_team = Team {
//             _id: team._id,
//             name: team.name,
//             city: team.city,
//             logo: team.logo,
//             is_favorite: true,
//         };

//         let fav_team = FavTeam {
//             _id: payload.user_id.to_owned(),
//             team: updated_team,
//         };

//         let status = match collection.insert_one(fav_team.clone(), None).await {
//             Ok(_) => Message {
//                 status: Status::Created.to_string(),
//                 message: "Favorite team created Successfully".to_owned(),
//             },
//             Err(_) => Message {
//                 status: Status::Conflict.to_string(),
//                 message: "Something went wrong, please try again".to_owned(),
//             },
//         };

//         status
//     }
