//Dependencies
use actix_web::{post, web, Responder};
use serde::{
    Serialize,
    Deserialize
};
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

#[derive(Serialize, Deserialize)]
pub struct SaveRequest {
    key: String,
    name: String,
    pasta: String
}

#[post("/save")]
pub async fn save(req: web::Json<SaveRequest>) -> impl Responder  {
    log(lvl::Info, &format!("Save Requested | '{}'", req.name));

    //Key check
    if !check_key(&req.key) {
        let res = Response::new(String::from("error"), Some(String::from("Invalid auth key")));
        return res.to_json();
    }
    
    //Add pasta to db
    let db = db::Database::new(String::from("./pastas.db"));
    let mut res: Response;
    match db.add(&req.name, &req.pasta) {
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
    use actix_web::{test, App, http};
    use dotenv::dotenv;
    use std::env;

    //Util function for tests to read the auth key.
    fn get_auth_key() -> String {
        //Reads test key from a .env file in the package root.
        dotenv().expect(".env file not found");
        let key: String = env::var("auth_key").unwrap();
        key
    }

    //Util function that resets the database after sending in data.
    fn reset_db() {
        match Database::new(String::from("./pastas.db")).reset(){ _ => {}}
    }

    /*
        This test function will test for a successful pasta save case.
    */
    #[actix_rt::test]
    async fn save_actix_success() {
        //Setup actix instance with the save endpoint
        let mut app = test::init_service(
            App::new()
                .service(save)
        ).await;
        
        //Create the request
        let req = test::TestRequest::post()
            .uri("/save")
            .set_json(&SaveRequest {
                key: get_auth_key(),
                name: "Avocado".to_owned(),
                pasta: "Guacamole".to_owned()
            })
        .to_request();

        //Set the expected response in byte data
        let expected_res = r##"{"status":"success","data":null}"##;

        //Run the request through the endpoint
        let res = test::call_service(&mut app, req).await;

        //Extract the response body, if not possible then panic
        let response_body = match res.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        //If response status is not OK, then panic
        assert_eq!(res.status(), http::StatusCode::OK);

        //If response body is not the expected response, then panic
        assert_eq!(response_body, expected_res);

        //Reset the database
        reset_db();
    }

    
    /*
        This test function will test for a case where a pasta with the same name
        already exists in the database.
    */
    #[actix_rt::test]
    async fn save_actix_duplicate_failure() {
        //Setup actix instance with the save endpoint
        let mut app = test::init_service(
            App::new()
                .service(save)
        ).await;
        
        //Create the request
        let req = test::TestRequest::post()
            .uri("/save")
            .set_json(&SaveRequest {
                key: get_auth_key(),
                name: "test".to_owned(),
                pasta: "This will fail since there is already a pasta called test in the database by default".to_owned()
            })
        .to_request();

        //Set the expected response in byte data
        let expected_res = r##"{"status":"fail","data":"UNIQUE constraint failed: pastas.name"}"##;

        //Run the request through the endpoint
        let res = test::call_service(&mut app, req).await;

        //Extract the response body, if not possible then panic
        let response_body = match res.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        //If response status is not OK, then panic
        assert_eq!(res.status(), http::StatusCode::OK);

        //If response body is not the expected response, then panic
        assert_eq!(response_body, expected_res);

        //Reset the database just in case
        reset_db();
    }
}