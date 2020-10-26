use mongodb::bson::{ Document, doc };
use mongodb::sync::{ Collection, Database };
use mongodb::options::*;
use mongodb::results::*;
use crate::db_connection::DatabaseConnection;
pub trait Model {
    fn get_collection(db: &Database) -> Collection;
  
    fn get_by_id(id: &str) -> Option<Document> {
        let db = DatabaseConnection::init();
        let collection = Self::get_collection(&db);
        collection.find_one(doc! { "_id": id}, None)
            .unwrap()
    }

    fn find_one(filter: Option<Document>, options: Option<FindOneOptions>) -> Option<Document> {
        let db = DatabaseConnection::init();
        let collection = Self::get_collection(&db);
        collection.find_one(filter, options)
            .unwrap()
    }
  
    fn find_many(filter: Option<Document>, options: Option<FindOptions>) -> Option<Vec<Document>> {
        let db = DatabaseConnection::init();
        let collection = Self::get_collection(&db);
        let cursor = collection.find(filter, options)
                                       .unwrap();
        let results = cursor.map(|doc| doc.unwrap())
                                           .collect();
        Some(results)
    }

    fn insert_one(new_doc: Document, options: Option<InsertOneOptions>) -> InsertOneResult {
        let db = DatabaseConnection::init();
        let collections = Self::get_collection(&db);
        collections.insert_one(new_doc.clone(), options)
                   .unwrap()
    }

    fn insert_many(new_docs: Vec<Document>, options: Option<InsertManyOptions>) -> InsertManyResult {
        let db = DatabaseConnection::init();
        let collections = Self::get_collection(&db);
        collections.insert_many(new_docs, options)
                   .unwrap()
    }

    fn delete_one(filter: Option<Document>, options: Option<DeleteOptions>) -> DeleteResult {
        let db = DatabaseConnection::init();
        let collections = Self::get_collection(&db);
        collections.delete_one(filter.unwrap(), options)
                   .unwrap()
    }
}