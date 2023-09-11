use rocket::{ http::Status};
use rocket::{Request,request::{Outcome, FromRequest}};

#[derive(Debug )]
pub struct HeaderAuth{
    pub token:String
}

// Implement FromRequest for the newtype
#[rocket::async_trait]

impl<'r> FromRequest<'r> for HeaderAuth {
    type Error = ();

   async  fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let header = request.headers().get_one("Authorization").unwrap_or("");
    if header==""{
        Outcome::Failure((Status::Unauthorized,()))
    }else{
            Outcome::Success(HeaderAuth{token:header.to_string()} )
         }
   }
   
}

 