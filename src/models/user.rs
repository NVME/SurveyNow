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
    pub created_at: NaiveDateTime
}

