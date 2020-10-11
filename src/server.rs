use rocket_contrib::json::Json;

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
    "Hello world!"
}

pub fn run_server() {
    rocket::ignite()
        .mount("/", routes![create])
        .mount("/", routes![read])
        .launch();
}