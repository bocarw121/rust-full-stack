use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Team {
    pub _id: u32,
    pub name: String,
    pub city: String,
    pub logo: String,
    pub is_favorite: bool
}

impl Team {
    pub fn default() -> Team {
        Team {
            _id: 0,
            name: "".to_owned(),
            city: "".to_owned(),
            logo: "".to_owned(),
            is_favorite: false
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct NBATeams {
    pub _id: String,
    pub teams: Vec<Team>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct NewTeam {
    pub name: String,
    pub city: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct FavTeam {
    pub _id: String,
    pub team: Vec<Team>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct FavTeamPayload {
    pub user_id: String,
    pub team_name: String
}



