#[macro_use]
extern crate rocket;

mod app;
mod modules;


use std::sync::atomic::AtomicUsize;

use app::{test_route::{test,test_auth}};
use modules::request_logger::RequestLogger;

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(("port", 1111));
    rocket::custom(figment).mount("/", routes![test,test_auth]).attach(RequestLogger {
        get: AtomicUsize::new(0),
        post: AtomicUsize::new(0),
        delete: AtomicUsize::new(0),
        put: AtomicUsize::new(0),
        patch: AtomicUsize::new(0),
    } )
    
}
