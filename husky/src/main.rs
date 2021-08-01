#[macro_use] extern crate rocket;
pub mod db;

//Save copypastas here.
#[get("/<name>/<value>")]
fn save(name: &str, value: &str) -> String {
    let db = db::Database::new(String::from("./pastas.db"));
    match db.add(name, value) {
        Ok(_) => println!("Successfully added new pasta '{}'", name),
        Err(db::PastaErr::DbErr(ref err)) => println!("Unsuccessful: {:?}", err)
    }
    
    format!("Saving '{}' as '{}'", value, name)
}

//Get copypastas here.
#[get("/<name>")]
fn send(name: &str) -> String {
    let db = db::Database::new(String::from("./pastas.db"));
    match db.get(name) {
        Ok(_) => println!("Success!"),
        Err(db::PastaErr::DbErr(ref err)) => println!("Unsuccessful: {:?}", err)
    }
    
    format!("Hello, {}", name)
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/save", routes![save])
        .mount("/send", routes![send])
}