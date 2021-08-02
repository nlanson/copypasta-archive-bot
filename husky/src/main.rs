#[macro_use] extern crate rocket;
pub mod db;
/*
    Todo:
        - Return data in JSON format with success status etc. (See jSend specification)
*/


//Save copypastas here.
#[get("/<name>/<value>")]
fn save(name: &str, value: &str) -> String {
    let db = db::Database::new(String::from("./pastas.db"));
    
    //Add pasta
    match db.add(name, value) {
        //Successful
        Ok(_) => println!("Successfully added new pasta '{}'", name),

        //Database SQLITE error
        Err(db::PastaErr::DbErr(ref err)) => println!("sqlite error: {:?}", err),

        //User error. Most likely a duplicate key
        Err(db::PastaErr::UsrErr(ref msg)) => println!("user error: {}", msg)
    }
    
    //Should return an error status code and error message any of the errors occur.
    format!("Saving '{}' as '{}'", value, name)
}

//Get copypastas here.
#[get("/<name>")]
fn send(name: &str) -> String {
    let db = db::Database::new(String::from("./pastas.db"));

    //Get pasta
    let val: String;
    match db.get(name) {
        //Success
        Ok(pasta) => val = pasta,

        //Database SQLITE error
        Err(db::PastaErr::DbErr(ref err)) => {
            println!("sqlite error: {:?}", err);
            val = String::from("sqlite error")
        },

        //User error. Should never happen as pastas that dont exists are covered in Database errors.
        Err(db::PastaErr::UsrErr(ref msg)) => { val = format!("user error: {:?}", msg) }
    }
    
    //Should return an error status code if val is sqlite error or user error.
    format!("{}", val)
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/save", routes![save])
        .mount("/send", routes![send])
}