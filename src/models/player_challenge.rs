use super::prelude::*;
use chrono::{ DateTime, Utc };

#[derive(Serialize, Deserialize, Debug)]
pub struct CompetitionPlayer {
   #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   id: Option<bson::oid::ObjectId>,
   player_id: bson::oid::ObjectId,
   challenge_id: bson::oid::ObjectId,
   complete: bool,
   timestamp_start: DateTime<Utc>,
   timestamp_end: DateTime<Utc>,
}

impl Model for CompetitionPlayer {
   fn get_collection(db: &Database) -> Collection {
       db.collection("playerChallenge")
   }
}