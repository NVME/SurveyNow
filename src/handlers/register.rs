use serde::Serialize;
use rocket_contrib::json::Json;
use rocket::response::status;
use rocket::http::Status;
use crate::repositories::Conn;
use crate::models::user::{ User};
use crate::repositories::user::{ NewUser};
use crate::handlers::ApplicationError;
use diesel::result::Error;

type ResponseCreated= Result<status::Created<Json<User>>,ApplicationError>;

#[post("/user", format = "application/json", data = "<user>")]
pub fn new_user(user: Json<NewUser>, conn: Conn) ->ResponseCreated{
   user.create(&conn)
   .map(|user| user_created(user))  
   .map_err(|error| match error {
       Error::DatabaseError(_kind, info)=>ApplicationError::Conflict((*info).message().to_string()),
       _=>ApplicationError::InternalError(String::from("Failed on creating new user, please try it again."))
   })    
 }

#[derive(Debug,Serialize,Deserialize)]
pub struct ResetPassword{ 
    pub id:i32,
    pub password:String
}

#[post("/user/reset_pw", format = "application/json", data = "<reset_pw>")]
pub fn reset_password(reset_pw: Json<ResetPassword>, conn: Conn) ->Option<Json<User>>{
    User::reset_password(reset_pw.id, reset_pw.password.clone(), &conn)
   .map(|user| Json(user))  
   .ok() 
 }

 fn user_created(user:User)->status::Created<Json<User>>{
    status::Created(
        format!("{host} :{port}/user/{id}", host="localhost", port="8000", id=user.id).to_string(),
        Some(Json(user))
    )
 }