use gloo_storage::{LocalStorage, Storage};
use reqwasm::http::Request;
use types::{FavTeamPayload, NBATeams, Team};
use uuid::Uuid;

use crate::components::favorite_teams::FavoriteTeams;
pub struct Fetch;

impl Fetch {

      pub async fn initialize_teams(user_id: String) {
        let url = format!("/nba/teams/insert?user_id={}", user_id);
        let res = match Request::get(&url).send().await {
            Ok(res) => res,
            Err(_) => panic!("Error fetching data"),
        };

        log::info!("Initialize {:?}", res.status().to_string());
    }
    pub async fn get_teams() -> NBATeams {
        let user_id = User::get_user_id();
        let url = format!("/nba/teams?user_id={}", user_id);
        let res = match Request::get(&url).send().await {
            Ok(res) => res,
            Err(e) => {panic!("Error fetching data{}", e)},
        };

        let teams: NBATeams = match res.json().await {
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
            Err(e) => {panic!("Error fetching data{}", e)},
        };

        let teams: Vec<types::FavTeam> = match res.json().await {
            Ok(teams) => teams,
            Err(_) => panic!("Error fetching data"),
        };

        teams
    }

  

    pub async fn get_team(url: String) -> Team {
        let res = match Request::get(&url).send().await {
            Ok(res) => res,
            Err(_) => panic!("Error fetching data"),
        };

        let team: Team = match res.json().await {
            Ok(team) => team,
            Err(_) => panic!("Error fetching data"),
        };

        team
    }

    pub async fn post_team(url: String, body: FavTeamPayload) {
        let body = serde_json::to_string(&body).unwrap();

        let result = match Request::post(&url)
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
