use rocket_contrib::json::Json;
use mongodb::bson::Document;
use crate::models::competition::Competition;
use crate::models::base_model::Model;

#[get("/")]
pub fn get_competitions() -> Json<Vec<Document>> {
    let a = Competition::find_many(None, None)
                                       .unwrap();
    Json(a)
}