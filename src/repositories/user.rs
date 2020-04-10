use diesel::pg::PgConnection;
use crate::models::user::User;
use crate::schema::users::dsl::*;
use crate::schema::users;
use diesel::prelude::*;
use diesel::{RunQueryDsl,QueryDsl};
use bcrypt::{ hash, BcryptResult, DEFAULT_COST};

#[derive(Debug, Serialize, Deserialize)]
pub struct Users(Vec<User>) ;

impl Users{

    pub fn all(conn: &PgConnection)-> Self {

        // let connection =establish_connection();
         let result= users.load::<User>(conn)
             .expect("Error loading users.");
         Users(result)
    }

  
}

impl User {
       
    pub fn get(uid:i32, conn:&PgConnection)->Result<User, diesel::result::Error>{
            users::table
                .filter(users::id.eq(uid))
                .first::<User>(conn) 
        }

    //http://diesel.rs/guides/all-about-updates/
    pub fn reset_password(uid:i32, new_password:String, conn:&PgConnection)-> Result<User,diesel::result::Error>{
        let updated=  diesel::update(users.find(id))
                .set(users::password.eq(hash(new_password, DEFAULT_COST).ok().expect("hash password failed")))
                .get_result(conn)
                .expect("reset password failed!");
                Ok(updated)
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub email:String,
    pub user_name:String,
    pub password:String
}

impl NewUser {
    
    pub fn create(&self,conn:&PgConnection)->Result<User, diesel::result::Error>{    
        
        let register= NewUser{
             email: self.email.clone(),
             user_name:self.user_name.clone(),
             password:hash(self.password.clone(), DEFAULT_COST).ok().expect("hash password failed")
        };      
        diesel::insert_into(users::table)
                  .values(register)
                  .get_result(conn)
    }
}



