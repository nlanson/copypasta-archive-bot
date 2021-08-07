//Dependencies and Modules
#[macro_use] extern crate rocket;
use serde::{Serialize, Deserialize};
mod db;
mod log;
use log::{
    Level as lvl,
    print as log
};

//Start server
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/save", routes![save])
        .mount("/send", routes![send])
}


//Save copypastas here
#[get("/<name>/<value>")]
fn save(name: &str, value: &str) -> String {
    log(lvl::Info, &format!("Save Requested | '{}'", name));
    
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
#[get("/<name>")]
fn send(name: &str) -> String {
    log(lvl::Info, &format!("Send Requested | '{}'", name));
    
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