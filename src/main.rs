
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate bcrypt;
extern crate jsonwebtoken as jwt;
#[macro_use] extern crate dotenv_codegen;
pub mod schema;
pub mod models;
pub mod handlers;
pub mod repositories;
use crate::handlers::{user, register, authentication};
use crate::repositories::Conn;
use rocket_cors::{ self ,Cors};


#[get("/")]
fn index()-> &'static str {
    " Hello , rocket!"
}
//https://github.com/lawliet89/rocket_cors/blob/master/examples/manual.rs
fn cors_fairing()->Cors {

    Cors::from_options(&Default::default()).expect("Cors fairing cannot be created")
}

fn main() {
    rocket::ignite()
           .attach(Conn::fairing())
           .attach(cors_fairing())
           .mount("/", 
                    routes![index,
                            user::get_users,
                            register::new_user,
                            user::get,
                            register::reset_password,
                            authentication::login
                            ])           
           .launch();
}
