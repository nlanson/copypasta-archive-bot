use serde::{Serialize, Deserialize};  
use dotenv::dotenv;
use std::env;
use crate::{
    log
};
use log::{
    Level as lvl,
    print as log
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub status: String,
    pub data: Option<String>
}

impl Response {
    pub fn new(status: String, data: Option<String>) -> Self {
        Self {
            status,
            data
        }
    }
    
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

//This function will check the key sent as part of the request and validate it against the hashed key stored below.
//If the key sent in hashes into the same stored hashed key the request can continue.
pub fn check_key(key: &str) -> bool {
    dotenv().expect(".env file not found");
    let hashed_key = format!("{:x}", md5::compute(env::var("auth_key").unwrap()));
    if format!("{:x}", md5::compute(key)) == hashed_key {
        true
    } else {
        //If auth key is not equal to the digest, then return an error
        log(lvl::Warning, "Invalid auth key detected.");
        false
    }
}

//Util functions used in unit tests.
pub mod testing_utils {
    use dotenv::dotenv;
    use std::env;
    use crate::db::Database;

    
    //Util function that resets the database after sending in data.
    //To be used in test modules
    pub fn reset_db() {
        match Database::new(String::from("./pastas.db")).reset(){ _ => {}}
    }

    //Read the auth key for test functions
    pub fn get_auth_key() -> String {
        //Reads test key from a .env file in the package root.
        dotenv().expect(".env file not found");
        let key: String = env::var("auth_key").unwrap();
        key
    }
}



#[cfg(test)]
mod tests{
    use super::*;
    use dotenv::dotenv;
    use std::env;

    fn get_auth_key() -> String {
        //Reads test key from a .env file in the package root.
        dotenv().expect(".env file not found");
        let key: String = env::var("auth_key").unwrap();
        key
    }

    #[test]
    fn key_check_success() {
        let key: String = get_auth_key();
        assert_eq!(true, check_key(&key));
    }

    #[test]
    fn key_check_failure() {
        let key: String = String::from("this is an incorrect string");
        assert_eq!(false, check_key(&key));
    }
}