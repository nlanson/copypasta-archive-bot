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
#[get("/<key>/<name>")]
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
    #[test]
    fn send_endpoint_success() {

    }

    #[test]
    fn send_endpoint_not_found_failure() {
        
    }
}