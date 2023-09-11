use crate::modules::response_handler::{CustomResult, CustomError, generic_response};


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