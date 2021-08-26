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

//Get copypastas here
//Request URL: http://<IPADDRESS>:8000/send/<key>/<name>
//#[get("/<key>/<name>")]
pub fn send(key: &str, name: &str) -> String {
    log(lvl::Info, &format!("Send Requested | '{}'", name));

    //Key check
    if !check_key(key) {
        let res = Response::new(String::from("error"), Some(String::from("Invalid auth key")));
        return res.to_json();
    }
    
    //Get pasta and send result.
    let db = db::Database::new(String::from("./pastas.db"));
    let mut res: Response;
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
    fn send_endpoint_success() {
        //Set up test data
        let key: String = get_auth_key();
        let name: String = "test".to_string();

        //Set expected response
        let expetected_res: String = Response::new("success".to_string(), Some("Hello World!".to_string())).to_json();

        //Run function and compare responses
        let res: String = send(&key, &name);
        assert_eq!(res, expetected_res);
    }

    #[test]
    fn send_endpoint_not_found_failure() {
        //Set up test data
        let key: String = get_auth_key();
        let name: String = "this pasta does not exist".to_string();

        //Set expected response
        let expetected_res: String = Response::new("fail".to_string(), Some("cannot read a text column".to_string())).to_json();

        //Run function and compare responses
        let res: String = send(&key, &name);
        assert_eq!(true, (res == expetected_res));
    }
}