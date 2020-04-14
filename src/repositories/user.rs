use diesel::pg::PgConnection;
use crate::models::user::User;
use crate::schema::users::dsl::*;
use crate::schema::users;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::result::{Error, DatabaseErrorKind};
use diesel::{RunQueryDsl,QueryDsl};
use bcrypt::{ hash, verify, BcryptResult, DEFAULT_COST};

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
       
    pub fn get(uid:i32, conn:&PgConnection)->Result<User,Error >{
            users::table
                .filter(users::id.eq(uid))
                .first::<User>(conn) 
        }
    //https://docs.diesel.rs/diesel/dsl/index.html
    pub fn check_email_duplication(this_email:&str,conn:&PgConnection) ->Result<bool, Error>{
        select(exists(users.filter(email.eq(this_email))))
        .get_result(conn)
    }

    //http://diesel.rs/guides/all-about-updates/
    pub fn reset_password(uid:i32, new_password:String, conn:&PgConnection)-> Result<User,Error>{
        let updated=  diesel::update(users.find(uid))
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
    
    pub fn create(&self,conn:&PgConnection)->Result<User, Error>{    
        
        let register= NewUser{
             email: self.email.clone(),
             user_name:self.user_name.clone(),
             password:hash(self.password.clone(), DEFAULT_COST).ok().expect("hash password failed")
        };  
       let duplicated= User::check_email_duplication(self.email.as_str(), conn)?;       
               
       if duplicated {                     
            return Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation,
                        Box::new(String::from("Duplicated email.Please consider other emails"))));
        }
        diesel::insert_into(users::table)
                  .values(register)
                  .get_result(conn)            
    }
}
#[derive(Deserialize)]
pub struct AuthUser {
    pub email:String,
    pub password:String
}

impl AuthUser {
    pub fn login(&self , conn:&PgConnection) ->Result<User ,Error>{
       //https://docs.diesel.rs/diesel/query_dsl/trait.RunQueryDsl.html
       let result= users::table
                    .filter(email.eq(&self.email))
                    .first::<User>(conn)?;
      
      let valid= verify(&self.password , &result.password).expect("validate password failed");
        
       if valid {
            Ok(result)
        } else{            
            Err(Error::NotFound)
        }       
    }
}
