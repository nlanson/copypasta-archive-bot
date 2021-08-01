#[macro_use] extern crate rocket;

//Save copypastas here.
#[get("/<name>/<value>")]
fn save(name: &str, value: &str) -> String {
    format!("Saving '{}' as '{}'", value, name)
}

//Get copypastas here.
#[get("/<name>")]
fn send(name: &str) -> String {
    format!("return: {}", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/save", routes![save])
        .mount("/send", routes![send])
}