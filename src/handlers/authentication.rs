use serde::Serialize;
use rocket_contrib::json::Json;
use crate::repositories::{Conn};
use crate::repositories::user::{ AuthUser };
use crate::models::user::User;
use diesel::result::Error;
use crate::handlers::ApplicationError;


#[derive(Debug, Serialize)]
pub struct AuthToken{

    pub token:String

}


#[post("/user/login", format="json", data="<user>")]
pub fn login(user:Json<AuthUser>, conn:Conn)->Result<Json<AuthToken>, ApplicationError>{

      user.login(&conn)
          .map(|user| Json(get_token(user).expect("get secrete / token failed")))
          .map_err(|error| match error {
                Error::NotFound=>ApplicationError::Unauthorized(String::from("Wrong password or user name/ email.")),
                _=>ApplicationError::InternalError(String::from("Failed on validating new user, please try it again."))
          })
}

fn get_token(user:User)->Result<AuthToken,String>{

    Ok(AuthToken{token:String::from("secrets")})
}