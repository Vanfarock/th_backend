use super::prelude::*;
use chrono::{ DateTime, Utc };

#[derive(Serialize, Deserialize, Debug)]
pub struct Competition {
   #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   id: Option<bson::oid::ObjectId>,
   timestamp_start: DateTime<Utc>,
   timestamp_end: DateTime<Utc>,
}

impl Model for Competition {
   fn get_collection(db: &Database) -> Collection {
       db.collection("competition")
   }
}