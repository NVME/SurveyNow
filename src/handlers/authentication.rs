use serde::Serialize;
use rocket_contrib::json::Json;
use crate::repositories::{Conn};
use crate::repositories::user::{ AuthUser };
use crate::models::user::{ User , RegisterUser };
use diesel::result::Error;
use crate::handlers::ApplicationError;
use crate::handlers::auth;



#[post("/user/login", format="json", data="<user>")]
pub fn login(user:Json<AuthUser>, conn:Conn)->Result<Json<RegisterUser>, ApplicationError>{

      user.login(&conn)
          .map(|user| Json(get_token(user).expect("get secrete / token failed")))
          .map_err(|error| match error {
                Error::NotFound=>ApplicationError::Unauthorized(String::from("Wrong password or user name/ email.")),
                _=>ApplicationError::InternalError(String::from("Failed on validating new user, please try it again."))
          })
}

fn get_token(user:User)->Result<RegisterUser,String>{
     let token= auth::create_token(user.email.as_str(), user.user_name.as_str(), user.is_admin)?;
     Ok(token)
}