//#[macro_use] extern crate rocket;
pub use actix_web::{App, HttpServer};


pub mod db;
pub mod server;
pub mod log;

//Rocket-rs Main launch method
// #[launch]
// fn rocket() -> _ {
//     let figment = rocket::Config::figment()
//         .merge(("address", "0.0.0.0"))
//         .merge(("log_level", "critical"));
    
//     rocket::custom(figment)  
//         .mount("/save", routes![server::save::save])
//         .mount("/send", routes![server::send::send])
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Copypasta archive bot database access API (v 26/8/21)");
    
    HttpServer::new(|| {
        App::new()
            .service(server::actix_save::save)
            .service(server::actix_send::send)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}