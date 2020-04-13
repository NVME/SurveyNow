use serde::Serialize;
use rocket_contrib::json::Json;
use crate::repositories::{Conn};
use crate::repositories::user::{ AuthUser };


#[post("/user/login", format="json", data="<user>")]
pub fn login(user:Json<AuthUser>, conn:Conn){

}