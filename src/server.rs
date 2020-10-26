use rocket_contrib::json::Json;
use crate::routes;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8
}

#[post("/", format="json", data="<person>")]
fn create(person: Json<Person>) -> Json<Person> {
    person
}

#[get("/")]
fn read() -> &'static str {
    "Hello world"
}

pub fn run() {
    rocket::ignite()
        .mount("/", routes::get_routes())
        .launch();
}