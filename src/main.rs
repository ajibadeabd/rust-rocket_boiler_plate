#[macro_use]
extern crate rocket;

mod app;
mod modules;

use std::env;
use std::sync::atomic::AtomicUsize;

use app::{test_route::{test,test_auth,test_token}};
use modules::request_logger::RequestLogger;
use dotenv::dotenv;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let port_str = env::var("PORT").unwrap();
    let port = port_str.parse::<i32>().expect("Failed to parse PORT as an integer");
    let figment = rocket::Config::figment()
        .merge(("port", port));

    rocket::custom(figment).mount("/", routes![
        test,
        test_auth,
        test_token
        ])
        .attach(RequestLogger {
        get: AtomicUsize::new(0),
        post: AtomicUsize::new(0),
        delete: AtomicUsize::new(0),
        put: AtomicUsize::new(0),
        patch: AtomicUsize::new(0),
    } )
    
}
