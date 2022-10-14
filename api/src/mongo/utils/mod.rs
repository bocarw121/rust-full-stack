use super::collections::team_collection;
use crate::rocket::futures::TryStreamExt;

pub async fn get_length_of_team() -> u32 {
    let collection = team_collection().await;

    // find all teams  if not just return and empty vec
    let cursor = match collection.find(None, None).await {
        Ok(cursor) => cursor,
        Err(_) => return 0,
    };

    // collect the cursor into a vector
    let teams = cursor.try_collect().await.unwrap_or_else(|_| vec![]);

    teams.len() as u32
}
