use crate::{
    server::util::{
        Response,
        check_key
    },
    db,
    log
};
use log::{
    Level as lvl,
    print as log
};

//Save copypastas here
//Request URL: http://<IPADDRESS>:8000/save/<key>/<name>/<value>
#[get("/<key>/<name>/<value>")]
pub fn save(key: &str, name: &str, value: &str) -> String {
    log(lvl::Info, &format!("Save Requested | '{}'", name));

    //Key check
    if !check_key(key) {
        let res = Response::new(String::from("error"), Some(String::from("Invalid auth key")));
        return res.to_json();
    }
    
    //Add pasta to db
    let db = db::Database::new(String::from("./pastas.db"));
    let mut res: Response;
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

            match &err.message {
                Some(msg) => {
                    res.data = Some(msg.clone());
                },
                None => { }
            }
        }
    }

    //Return json string
    res.to_json()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;
    use crate::server::util::Response;
    use dotenv::dotenv;
    use std::env;

    fn get_auth_key() -> String {
        //Reads test key from a .env file in the package root.
        dotenv().expect(".env file not found");
        let key: String = env::var("auth_key").unwrap();
        key
    }
    
    #[test]
    fn save_endpoint_success() {
        //Setup test data
        let key: String = get_auth_key();
        let name: String = String::from("testdata");
        let pasta: String = String::from("This is a test pasta");

        //Set exprected response
        let expected_res: String = Response::new(String::from("success"), None).to_json();

        //Run function and compare response to expected response
        let res: String = save(&key, &name, &pasta);
        assert_eq!(res, expected_res);

        //Reset database
        match Database::new(String::from("./pastas.db")).reset(){ _ => {}}
    }

    #[test]
    fn save_endpoint_duplicate_failure() {
        //Setup test data
        let key: String = get_auth_key();
        let name: String = String::from("test");
        let pasta: String = String::from("This is a test pasta");

        //Set exprected response
        let expected_res: String = Response::new(String::from("fail"), Some(String::from("UNIQUE constraint failed: pastas.name"))).to_json();

        //Run function and compare response to expected response
        let res: String = save(&key, &name, &pasta);
        assert_eq!(res, expected_res);

        //Reset database
        match Database::new(String::from("./pastas.db")).reset(){ _ => {}}

    }
}