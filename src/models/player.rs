use super::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Player<'a> {
   #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   id: Option<bson::oid::ObjectId>,
   name: &'a str
}

impl <'a>Model for Player<'a> {
   fn get_collection(db: &Database) -> Collection {
       db.collection("player")
   }
}