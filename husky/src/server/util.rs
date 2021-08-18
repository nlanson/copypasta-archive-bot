use serde::{Serialize, Deserialize};
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
    let hashed_key = "578fb4d629c3a508df141858e20bcdb3";
    if format!("{:x}", md5::compute(key)) == hashed_key {
        true
    } else {
        //If auth key is not equal to the digest, then return an error
        log(lvl::Warning, "Invalid auth key detected.");
        false
    }
}