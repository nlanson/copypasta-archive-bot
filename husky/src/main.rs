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
    let flag: bool;
    match db.add(name, value) {
        //Successful
        Ok(_) => {
            println!("Successfully added new pasta '{}'", name);
            flag = true;
        },

        //Database SQLITE error
        Err(db::PastaErr::DbErr(ref err)) => {
            println!("Sqlite Error Code: {:?} | Message: {:?}", err.code, err.message);
            flag = false;
        }
    }
    

    if flag {
        format!("Successful")
    } else {
        format!("Unsuccessful")
    }
    
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
            println!("Sqlite Error Code: {:?} | Message: {:?}", err.code, err.message);
            val = String::from("sqlite error")
        }
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