//Dependencies
#[macro_use] extern crate rocket;
use serde::{Serialize, Deserialize};

//Modules
mod db;
mod log;
use log::{
    Level as lvl,
    print as log
};

//Start server
#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("log_level", "critical"));
    
    rocket::custom(figment)  
        .mount("/save", routes![save])
        .mount("/send", routes![send])
}

//Save copypastas here
#[get("/<key>/<name>/<value>")]
fn save(key: &str, name: &str, value: &str) -> String {
    log(lvl::Info, &format!("Save Requested | '{}'", name));

    //Key check
    if !check_key(key) {
        let res = Response::new(String::from("error"), Some(String::from("Invalid auth key")));
        return res.to_json();
    }
    
    //Add pasta to db
    let db = db::Database::new(String::from("./pastas.db"));
    let res: Response;
    match db.add(name, value) {
        //Successful
        Ok(_) => {
            log(lvl::Info, "Success");
            res = Response::new(String::from("success"), None);
        },

        //Database SQLITE error
        Err(db::PastaErr::DbErr(ref err)) => {
            log(lvl::Error, &format!("Failed | {:?}", err.message));
            res = Response::new(String::from("fail"), None);
        }
    }

    //Return json string
    res.to_json()
}

//Get copypastas here
#[get("/<key>/<name>")]
fn send(key: &str, name: &str) -> String {
    log(lvl::Info, &format!("Send Requested | '{}'", name));

    //Key check
    if !check_key(key) {
        let res = Response::new(String::from("error"), Some(String::from("Invalid auth key")));
        return res.to_json();
    }
    
    //Get pasta and send result.
    let db = db::Database::new(String::from("./pastas.db"));
    let res: Response;
    match db.get(name) {
        //Success
        Ok(pasta) => {
            log(lvl::Info, "Success");
            res = Response::new(String::from("success"), Some(pasta));
        },

        //Database SQLITE error
        Err(db::PastaErr::DbErr(ref err)) => {
            log(lvl::Error, &format!("Failed | {:?}", err.message));
            res = Response::new(String::from("fail"), None);
        }
    }
    
    //Return json string
    res.to_json()
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    status: String,
    data: Option<String>
}

impl Response {
    pub fn new(status: String, data: Option<String>) -> Self {
        Self {
            status,
            data
        }
    }
    
    pub fn to_json(&self) -> String{
        serde_json::to_string(&self).unwrap()
    }
}

fn check_key(key: &str) -> bool {
    let hashed_key = "578fb4d629c3a508df141858e20bcdb3";
    if format!("{:x}", md5::compute(key)) == hashed_key {
        //If auth key is not equal to the digest, then return an error
        log(lvl::Warning, "Invalid auth key detected.");
        true
    } else {
        false
    }
}