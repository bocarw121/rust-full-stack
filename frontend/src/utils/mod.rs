
use reqwasm::http::Request;
use types::Team;
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

}
