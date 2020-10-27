use super::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct CompetitionPlayer {
   #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   id: Option<bson::oid::ObjectId>,
   player_id: bson::oid::ObjectId,
   competition_id: bson::oid::ObjectId,
   total_points: u32
}

impl Model for CompetitionPlayer {
   fn get_collection(db: &Database) -> Collection {
       db.collection("competitionPlayer")
   }
}