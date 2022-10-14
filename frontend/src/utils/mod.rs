use gloo_storage::{LocalStorage, Storage};
use reqwasm::http::Request;
use serde::{Serialize, Deserialize};
use types::{FavTeamPayload, NBATeams, Team};
use uuid::Uuid;


#[derive(Debug,Deserialize)]
pub struct JsonResponse<T> {
 pub status: String,
 pub data: T
}

pub struct Fetch;

impl Fetch {
    pub async fn initialize_teams(user_id: String) {
        let url = format!("/nba/teams/insert?user_id={}", user_id);
        let res = match Request::get(&url).send().await {
            Ok(res) => res,
            Err(_) => panic!("Error fetching data"),
        };

    }
    pub async fn get_teams() -> JsonResponse<Vec<NBATeams>> {
        let user_id = User::get_user_id();
        let url = format!("/nba/teams?user_id={}", user_id);
        let res = match Request::get(&url).send().await {
            Ok(res) => res,
            Err(e) => {
                panic!("Error fetching data{}", e)
            }
        };

        let teams: JsonResponse<Vec<NBATeams>>  = match res.json().await {
            Ok(teams) => teams,
            Err(_) => panic!("Error fetching data"),
        };

        teams
    }

    pub async fn get_fav_teams() -> Vec<types::FavTeam> {
        let user_id = User::get_user_id();
        let url = format!("/nba/favorite?user_id={}", user_id);
        let res = match Request::get(&url).send().await {
            Ok(res) => res,
            Err(e) => {
                panic!("Error fetching data{}", e)
            }
        };

        let teams: Vec<types::FavTeam> = match res.json().await {
            Ok(teams) => teams,
            Err(_) => panic!("Error fetching data"),
        };

        teams
    }

    pub async fn update_favorite_team(team_name: String) {
        let user_id = User::get_user_id();
        let url = format!("/nba/teams?team_name={}&user_id={}", team_name, user_id);
        let response = match Request::put(&url).send().await {
            Ok(response) => response,
            Err(_) => panic!("Error adding favorites"),
        };
    }


    pub async fn get_team(url: String) -> JsonResponse<Team> {
        let res = match Request::get(&url).send().await {
            Ok(res) => res,
            Err(_) => panic!("Error fetching data"),
        };

        let team = match res.json().await {
            Ok(team) => team,
            Err(_) => panic!("Error fetching data"),
        };

        team
    }

    pub async fn post_fav_team(team_name: String) {
        let user_id = User::get_user_id();
        let body = FavTeamPayload {
            user_id,
            team_name,
        };

        let body = serde_json::to_string(&body).unwrap();
        
        let result = match Request::post("/nba/favorite")
            .body(&body)
            .header("Content-Type", "application/json")
            .send()
            .await
        {
            Ok(res) => res,
            Err(_) => panic!("Error posting fetching data"),
        };

        log::info!("Post body {:?}", &body.to_string());
    }


    pub async fn delete_favorite_team(team_name: String)  {
        let user_id = User::get_user_id();
        let url = format!("/nba/favorite?team_name={}&user_id={}", team_name, user_id);
            match Request::delete(&url).send().await {
            Ok(response) => response,
            Err(_) => panic!("Error adding favorites"),
        };
    }

    
}

pub struct User;

impl User {
    pub fn set_user_id() {
        let generated_id = Uuid::new_v4();

        match LocalStorage::set("user_id", generated_id.to_string()) {
            Ok(value) => value,
            Err(_) => panic!("Unable to set user"),
        };
    }

    pub fn get_user_id() -> String {
        let user_id: String = match LocalStorage::get("user_id") {
            Ok(value) => value,
            Err(_) => "".to_owned(),
        };
        user_id
    }
}
