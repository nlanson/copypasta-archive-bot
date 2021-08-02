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
    let val: String;
    match db.get(name) {
        Ok(pasta) => val = pasta,
        Err(db::PastaErr::DbErr(ref err)) => {
            println!("Unsuccessful: {:?}", err);
            val = String::from("db get unsuccessful")
        }
    }
    
    format!("{}", val)
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/save", routes![save])
        .mount("/send", routes![send])
}