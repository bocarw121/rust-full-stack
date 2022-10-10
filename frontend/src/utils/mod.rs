use reqwasm::http::Request;
use types::{Team, FavTeamPayload};
pub struct Fetch;

impl Fetch {
    pub async fn get_teams(url: String) -> Vec<Team> {
        let res = match Request::get(&url).send().await {
            Ok(res) => res,
            Err(_) => panic!("Error fetching data"),
        };

        let teams: Vec<Team> = match res.json().await {
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

    pub async fn post_team(url: String, body: FavTeamPayload){
        let body = serde_json::to_string(&body).unwrap();
       
       let result = match  Request::post(&url).body(&body).header("Content-Type", "application/json").send().await {
            Ok(res) => res,
            Err(_) =>  panic!("Error posting fetching data")
        };


          log::info!("Post body {:?}", &body.to_string());
    }
}
