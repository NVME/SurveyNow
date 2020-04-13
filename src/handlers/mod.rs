pub mod user;
pub mod register;
pub mod authentication;

//https://api.rocket.rs/v0.4/rocket/derive.Responder.html
#[derive(Responder, Debug)]
pub enum ApplicationError {
    #[response(status = 500, content_type = "json")]
    InternalError(String),
    #[response(status = 409, content_type = "json")] 
    Conflict(String)  
}