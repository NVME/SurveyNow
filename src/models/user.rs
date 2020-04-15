use crate::schema::users;
use chrono::NaiveDateTime;
#[derive( Debug , Serialize , Deserialize ,Insertable, Queryable)]
#[table_name="users"]
pub struct User {
    #[serde(skip)]
    pub id: i32 ,
    pub user_name:String ,
    pub email:String,  
    #[serde(skip)]
    pub password:String, 
    pub created_at: NaiveDateTime,
    pub is_admin:bool, 
}

#[derive(Debug , Serialize , Deserialize)]
pub struct RegisterUser {
    pub user_name:String ,
    pub email: String ,
    pub token: String,
    pub expired_at:NaiveDateTime
}

#[derive(Debug , Serialize , Deserialize)]
pub struct ValidUser {
    pub user_name:String ,
    pub email: String
}

#[derive(Debug , Serialize , Deserialize)]
pub struct AdminUser {
    pub user_name:String ,
    pub email: String,
    pub is_admin:bool
}




