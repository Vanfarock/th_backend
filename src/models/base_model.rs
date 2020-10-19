use mongodb::bson::Document;
use mongodb::{ Collection, Database };
use mongodb::options::*;
use mongodb::results::*;

use async_trait::async_trait;

#[async_trait]
pub trait Model {
    fn get_collection(db: &Database) -> Collection;

    async fn get_by_id(id: &str) -> Option<Document>;
    async fn find_one(filter: Option<Document>, options: Option<FindOneOptions>) -> Option<Document>;
    async fn find_many(filter: Option<Document>, options: Option<FindOptions>) -> Option<Vec<Document>>;
    
    async fn insert_one(doc: Document, options: Option<InsertOneOptions>) -> InsertOneResult;
    async fn insert_many(docs: Vec<Document>, options: Option<InsertManyOptions>) -> InsertManyResult;

    async fn delete_one(filter: Option<Document>, options: Option<DeleteOptions>) -> DeleteResult;
}