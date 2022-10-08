use mongodb::Collection;
use types::{FavTeamWithId, Team};

use super::db;

pub const TEAM_COLLECTION: &str = "teams";
pub const FAV_TEAM_COLLECTION: &str = "fav-team";

pub async fn team_collection() -> Collection<Team> {
    let db = db::create().await;

    let collection = db.collection::<Team>(TEAM_COLLECTION);

    collection
}

pub async fn fav_team_collection() -> Collection<FavTeamWithId> {
    let db = db::create().await;

    let collection = db.collection::<FavTeamWithId>(FAV_TEAM_COLLECTION);

    collection
}
