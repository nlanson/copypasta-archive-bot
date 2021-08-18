#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

pub mod db;
pub mod server;
pub mod log;

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("log_level", "critical"));
    
    rocket::custom(figment)  
        .mount("/save", routes![server::save::save])
        .mount("/send", routes![server::send::send])
}