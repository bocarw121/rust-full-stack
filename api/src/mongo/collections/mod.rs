use mongodb::Collection;
use types::{FavTeam, NBATeams, Team};

use super::db;

pub const TEAM_COLLECTION: &str = "teams";
pub const FAV_TEAM_COLLECTION: &str = "fav-team";

pub async fn team_collection() -> Collection<NBATeams> {
    let db = db::create().await;

    let collection = db.collection::<NBATeams>(TEAM_COLLECTION);

    collection
}

pub async fn fav_team_collection() -> Collection<FavTeam> {
    let db = db::create().await;

    let collection = db.collection::<FavTeam>(FAV_TEAM_COLLECTION);

    collection
}
