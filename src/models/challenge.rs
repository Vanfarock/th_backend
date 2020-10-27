use super::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct CompetitionPlayer<'a> {
   #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   id: Option<bson::oid::ObjectId>,
   description: &'a str,
   sub_type: bson::oid::ObjectId,
   points: u32
}

impl <'a>Model for CompetitionPlayer<'a> {
   fn get_collection(db: &Database) -> Collection {
       db.collection("challenge")
   }
}