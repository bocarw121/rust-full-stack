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

    pub async fn get_one_team(name: String, user_id: String) -> Result<Team, Message> {
        let doc = doc! {"_id": user_id};
           let cursor = match team_collection().await.find(doc, None).await {
            Ok(cursor) => cursor,
            Err(_) => return Err(Message {
                status: Status::NotFound.to_string(),
                message: "Team not found".to_owned()
            })
        };

        let team_iter = cursor
            .try_collect()
            .await
            .unwrap_or_else(|_| vec![])
            .into_iter();

        let team_vec = team_iter.into_iter().flat_map(|teams| teams.teams.into_iter()).collect::<Vec<Team>>();
        
        let team = match team_vec.into_iter().find(|team| team.name.to_lowercase() == name.to_lowercase())  {
            Some(team) => team,
            None => return Err(Message {
                status: Status::NotFound.to_string(),
                message: "Team not found".to_owned()
            })
        }; 
   
     Ok(team)


    }

       

    pub async fn add_favorite_team(payload: Json<FavTeamPayload>) -> Message {
        let collection = fav_team_collection().await;

        // Use the get one team model to get the teams
        let team = match Self::get_one_team(payload.team_name.clone(), payload.user_id.clone()).await {
            Ok(team) => team,
            Err(_) =>return Message {
                status: Status::NotFound.to_string(),
                message: "Team not found".to_owned()
            }
        }; 

        let updated_team = Team {
            _id: team._id,
            name: team.name,
            city: team.city,
            logo: team.logo,
            // This will toggle is_favorite state
            is_favorite: true,
        };

        let fav_team = vec![ 
            FavTeam {
            _id: payload.user_id.to_owned(),
            team: updated_team,
        }
        
        ];

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

    pub async fn get_all_favorite_teams(user_id: String) -> Vec<FavTeam> {
        let collection = fav_team_collection().await;

        let fav_teams = match collection.find(doc! {"_id": user_id}, None).await {
            Ok(cursor) => cursor,
            Err(_) => panic!("Error getting favorite teams"),
        };

        fav_teams.try_collect().await.unwrap_or_else(|_| vec![]).into_iter().flat_map(|fav_teams|fav_teams.into_iter()).collect::<Vec<FavTeam>>()


      
    }


     pub async fn update_one_team(name: String, user_id: String) -> Message {
        let team = Self::get_one_team(name.clone(), user_id.clone()).await;


        let team = match team {
            Ok(team) => team,
            Err(_) => return Message {
            status: Status::InternalServerError.to_string(),
            message: "Something went wrong please try again.".to_string()
            }
        };

        let collection = team_collection().await;
        
        // This will access the correct index for the nba team that needs to be updated
        let team_update_key = format!("teams.{}.is_favorite", team._id -1);

   
        
      let result = match collection.update_one(doc!{"_id": user_id}, 
        doc! {"$set": {team_update_key:!team.is_favorite}}, None).await {
            Ok(result) => result,
            Err(_) => return Message {
            status: Status::InternalServerError.to_string(),
            message: "We were unable to update your favorite teams".to_string()
            }
        };

        let message = if team.is_favorite == true {
            format!("You successfully added {} to favorites", team.name)
        } else {
            format!("You successfully removed {} from favorites", team.name)
        };

        Message {
            status: Status::Ok.to_string(),
            message
        }

        

        
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
