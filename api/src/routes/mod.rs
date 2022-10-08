
use rocket::{serde::json::Json, futures::TryStreamExt};
use types::{NBATeams, NewTeam, Team};

use crate::mongo::Model;

#[get("/")]
pub(crate) fn index() -> &'static str {
    "Welcome to the NBA teams api"
}

#[get("/teams")]
pub(crate) async fn all_nba_teams() -> Json<NBATeams> {
 

    // find all teams  if not just return and empty vec
    let teams = Model::get_all_teams().await;

    println!("{:?}", teams);

    Json(NBATeams {
        teams,
    })
}

#[get("/teams/<name>")]
pub(crate) async fn get_team_by_name(name: String) -> Result<Json<Team>, String> {
    let db = Model::get_collection("teams".to_owned()).await;

    let cursor = match db.find(None, None).await {
        Ok(cursor) => cursor,
        Err(_) => return Ok(Json(Team::default())),
    };

    let team = match cursor
        .try_collect()
        .await
        .unwrap_or_else(|_| vec![])
        .into_iter()
        .find(|team| team.name.to_lowercase() == name)
    {
        Some(team) => Json(team),
        None => return Err("No team with that name".to_owned()),
    };

    Ok(team)
}

#[post("/teams", format = "application/json", data = "<newteam>")]
pub async fn make_a_team(newteam: Json<NewTeam>) -> Json<String> {
    let new_team = NewTeam {
        name: newteam.name.to_owned(),
        city: newteam.city.to_owned(),
    };



    // random id for new team
    let id = Model::get_length_of_items().await + 1;

    let new_team_with_id = Team {
        _id: id,
        name: new_team.name.to_owned(),
        city: new_team.city.to_owned(),
    };

    let collection = Model::get_collection("teams".to_owned()).await;

    let result = match collection.insert_one(new_team_with_id, None).await {
        Ok(result) => Json(format!(
            "Team added to database: result id: {}",
            result.inserted_id.to_string()
        )),
        Err(e) => return Json(format!("Error {}", e.kind)),
    };
    result
}
