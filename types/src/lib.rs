use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Team {
    pub _id: u32,
    pub name: String,
    pub city: String,
}

impl Team {
   pub fn default() -> Team {
        Team { _id: 0, name: "".to_owned(), city: "".to_owned() }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct NBATeams {
    pub teams: Vec<Team>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct NewTeam {
   pub name: String,
   pub city: String,
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct FavTeam {
   pub user_name: String,
   pub team: Team,
}

