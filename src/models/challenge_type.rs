use super::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeType<'a> {
   #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   id: Option<bson::oid::ObjectId>,
   name: &'a str,
   sub_type: Option<bson::oid::ObjectId>
}

impl <'a>Model for ChallengeType<'a> {
   fn get_collection(db: &Database) -> Collection {
       db.collection("challengeType")
   }
}