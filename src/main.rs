#[macro_use]
extern crate rocket;

mod app;
mod modules;

use std::env;
use app::{test_route::{test,test_auth,test_token}};
use modules::request_logger::{logger};
use dotenv::dotenv;
use crate::modules::cors::make_cors;
use crate::modules::error_handler::{internal_error,bad_input,not_found,not_authorize};

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
        test_token,
        ])
    .register("/", catchers![
        not_authorize,
        internal_error, 
        not_found, 
        bad_input
        ])
        .attach(logger() ).attach(make_cors())
}
