#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::{ Json };

#[derive(Serialize, Deserialize, Debug)]
struct Challenge {
    tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    name: String
}


#[post("/create-challenges", format="json", data = "<challenges>")]
fn index(challenges: Json<Challenge>) -> Json<Challenge> {
    let tasks = challenges.0;
    println!("{:#?}", tasks);
    Json(tasks)
}

fn main() {
    rocket::ignite().mount("/", routes![ 
                                        index,
                                        ]).launch();
}