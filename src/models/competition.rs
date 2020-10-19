use mongodb::{ bson, Collection, Database };
use mongodb::bson:: { Document, doc };
use mongodb::options::*;
use mongodb::results::*;
use chrono::prelude::*;
use serde::{ Serialize, Deserialize };
use async_trait::async_trait;
use futures::StreamExt;

use super::base_model::Model;
use crate::db_connection::DatabaseConnection;

#[derive(Serialize, Deserialize, Debug)]
pub struct Competition {
   #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   id: Option<bson::oid::ObjectId>,
   timestamp_start: DateTime<Utc>,
   timestamp_end: DateTime<Utc>,
}

#[async_trait]
impl Model for Competition {
   fn get_collection(db: &Database) -> Collection {
       db.collection("competition")
   }

   async fn get_by_id(id: &str) -> Option<Document> {
      Self::find_one(Some(doc! {"_id": id }), None).await
   }

   async fn find_one(filter: Option<Document>, options: Option<FindOneOptions>) -> Option<Document> {
      let db = DatabaseConnection::init().await;
      let competition = Self::get_collection(&db);
      competition.find_one(filter, options)
         .await
         .expect("Could not find a competition with given filter")
   }

   async fn find_many(filter: Option<Document>, options: Option<FindOptions>) -> Option<Vec<Document>> {
      let db = DatabaseConnection::init().await;
      let competition = Self::get_collection(&db);
      let cursor = competition.find(filter, options)
                                      .await
                                      .expect("Error while trying to find competitions");
      let competitions = cursor.map(|doc| doc.unwrap()).collect().await;
      Some(competitions)
   }

   async fn insert_one(new_doc: Document, options: Option<InsertOneOptions>) -> InsertOneResult {
      let db = DatabaseConnection::init().await;
      let competition = Self::get_collection(&db);
      competition.insert_one(new_doc.clone(), options)
                 .await
                 .expect("Error while inserting a new competition")
   }

   async fn insert_many(new_docs: Vec<Document>, options: Option<InsertManyOptions>) -> InsertManyResult {
      let db = DatabaseConnection::init().await;
      let competition = Self::get_collection(&db);
      competition.insert_many(new_docs, options)
                 .await
                 .expect("Error while inserting many competitions")
   }

   async fn delete_one(filter: Option<Document>, options: Option<DeleteOptions>) -> DeleteResult {
      let db = DatabaseConnection::init().await;
      let competition = Self::get_collection(&db);
      competition.delete_one(filter.unwrap(), options)
                 .await
                 .expect("Error while deleting a competition")
   }
}