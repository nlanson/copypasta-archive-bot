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
pub struct SendRequest {
    key: String,
    name: String
}

#[post("/send")]
pub async fn send(req: web::Json<SendRequest>) -> impl Responder {
    log(lvl::Info, &format!("Send Requested | '{}'", req.name));

    //Key check
    if !check_key(&req.key) {
        let res = Response::new(String::from("error"), Some(String::from("Invalid auth key")));
        return res.to_json();
    }
    
    //Get pasta and send result.
    let db = db::Database::new(String::from("./pastas.db"));
    let mut res: Response;
    match db.get(&req.name) {
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
    use crate::server::util;
    use actix_web::{test, App, http};

    #[actix_rt::test]
    async fn send_actix_success() {
        //Setup actix instance with the save endpoint
        let mut app = test::init_service(
            App::new()
                .service(send)
        ).await;
        
        //Create the request
        let req = test::TestRequest::post()
            .uri("/send")
            .set_json(&SendRequest {
                key: util::testing_utils::get_auth_key(),
                name: "test".to_owned(),
            })
        .to_request();

        //Set the expected response in byte data
        let expected_res = r##"{"status":"success","data":"Hello World!"}"##;

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
    }

    #[actix_rt::test]
    async fn send_actix_not_found_failure() {
        //Setup actix instance with the save endpoint
        let mut app = test::init_service(
            App::new()
                .service(send)
        ).await;
        
        //Create the request
        let req = test::TestRequest::post()
            .uri("/send")
            .set_json(&SendRequest {
                key: util::testing_utils::get_auth_key(),
                name: "This pasta does not exist and therefor, this test will fail".to_owned(),
            })
        .to_request();

        //Set the expected response in byte data
        let expected_res = r##"{"status":"fail","data":"cannot read a text column"}"##;

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
    }
}