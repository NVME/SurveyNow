
use serde::Serialize;
use rocket_contrib::json::Json;
use rocket::response::status;
use rocket::http::Status;
use crate::repositories::Conn;
use crate::models::user::{ User};
use crate::repositories::user::{ Users};



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



