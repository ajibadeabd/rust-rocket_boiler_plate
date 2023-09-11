use rocket::Request;
use super::response_handler::CustomError;

// This function handles 404 Not Found errors.
#[catch(404)]
pub fn not_found(req: &Request) -> CustomError {
    // Generate a response message for the error.
    let response = format!("I couldn't find '{}'. Try something else?", req.uri());

    // Create a CustomError with a NotFound variant and the response message.
    CustomError::NotFound(response)
}

// This function handles 422 Unprocessable Entity errors.
#[catch(422)]
pub fn bad_input(req: &Request) -> CustomError {
    // Generate a response message for the error.
    let response = format!("Please provide valid input");

    // Create a CustomError with a BadInput variant and the response message.
    CustomError::BadInput(response)
}

// This function handles 500 Internal Server Error errors.
#[catch(500)]
pub fn internal_error() -> CustomError {
    // Generate a response message for the error.
    let response = format!("Whoops! Looks like we messed up.");

    // Create a CustomError with an Internal variant and the response message.
    CustomError::Internal(response)
}

// This function handles 401 Unauthorized errors.
#[catch(401)]
pub fn not_authorize(req: &Request) -> CustomError {
    // Extract the URI from the request.
    let uri = req.uri();

    // Create a more informative error message.
    let response = format!("Access to '{}' is unauthorized. Please log in or provide valid credentials.", uri);

    // Create a CustomError with an Unauthorized variant and the response message.
    CustomError::Unauthorized(response)
}
