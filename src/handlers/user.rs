
use serde::Serialize;
use rocket_contrib::json::Json;
use rocket::response::status;
use rocket::http::Status;
use crate::repositories::Conn;
use crate::models::user::{ User };
use crate::repositories::user::{ Users, NewUser};

type ResponseCreated= Result<status::Created<Json<User>>,Status>;

#[get("/users")]
pub fn get_users(conn: Conn)->Json<Users>{
    Json(Users::all(&conn))
}

#[get("/user/<id>")]
pub fn get(id:i32, conn:Conn) ->Option<Json<User>>{
    User::get(id, &conn)
      .map(|user|Json(user))
      .ok()
      //.map( |user|Some(Json(user))).unwrap_or(None)
      // .map_or(None, |user|Some(Json(user)))
}


#[post("/user", format = "application/json", data = "<user>")]
pub fn new_user(user: Json<NewUser>, conn: Conn) ->ResponseCreated{
   user.create(&conn)
   .map(|user| user_created(user))  
   .map_err(|_error| Status::InternalServerError )    
 }

#[post("/user/<uid>/reset_pw", format = "application/json", data = "<user>")]
pub fn reset_password(uid:i32, user: Json<NewUser>, conn: Conn) ->Option<Json<User>>{
    User::reset_password(uid, user.password.clone(), &conn)
   .map(|user| Json(user))  
   .ok() 
 }

 fn user_created(user:User)->status::Created<Json<User>>{
    status::Created(
        format!("{host} :{port}/user/{id}", host="localhost", port="8000", id=user.id).to_string(),
        Some(Json(user))
    )
 }
