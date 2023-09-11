use chrono::{Duration, Utc};

use crate::modules::{response_handler::{CustomResult, CustomError, generic_response}, util::{encode_jwt, encode_token_and_refresh}};


#[get("/")]
pub async fn test(
)  -> Result<CustomResult, CustomError>{

   let  response = generic_response::<Option<String>>(
        "non auth route tested  successfully.",
       None,
       None
   );
   Ok(response)
}

#[get("/auth")]
pub async fn test_auth(
)  -> Result<CustomResult, CustomError>{
    let  response = generic_response::<Option<String>>(
        "non auth route tested  successfully.",
       None,
       None
   );
   Ok(response)
}

#[get("/get_token")]
pub async fn test_token(
)  -> Result<CustomResult, CustomError>{
    let current_time = Utc::now();

    // Add 2 hours to the current time
    let two_hours_later = current_time + Duration::hours(2);
    let two_week_later = current_time + Duration::weeks(2);
    let duration_seconds = (two_hours_later - current_time).num_seconds();
    let duration_of_days_seconds = (two_week_later - current_time).num_seconds();
    

    let token = encode_token_and_refresh(
        "fake_user_id".to_owned(),
     "secret","",
     duration_of_days_seconds,
     duration_seconds).unwrap();

    let  response = generic_response(
        "non auth route tested  successfully.",
        Some(token),
       None
   );
   Ok(response)
}