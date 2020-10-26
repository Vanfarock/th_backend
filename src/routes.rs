use crate::controllers::competition_controller;
use rocket::Route;

pub fn get_routes() -> Vec<Route> {
    return routes![competition_controller::get_competitions];
}

