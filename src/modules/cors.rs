use rocket_cors::{
    CorsOptions, AllowedHeaders, AllowedOrigins,
    Cors as RocketCors
};
 
use std::str::FromStr;
use rocket_cors::AllowedMethods;
 
 

pub fn make_cors() -> RocketCors {
    let allowed_origins = AllowedOrigins::all();
    let allowed_methods: AllowedMethods = ["Get", "Post", "Delete"]
        .iter()
        .map(|s| FromStr::from_str(s).unwrap())
        .collect();
    let cors_options = CorsOptions {
        allowed_origins,
        allowed_methods,
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    };
    RocketCors::from_options(&cors_options).expect("Error building CORS fairing")
}