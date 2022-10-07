use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Team {
    pub id: u32,
    pub team: String,
    pub city: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct NBATeams {
    pub teams: Vec<Team>,
}
