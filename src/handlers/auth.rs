use jwt::{decode, encode ,Header, Validation};
use chrono::{ Local, Duration, NaiveDateTime};
use crate::models::user:: { RegisterUser, ValidUser, AdminUser };
use rocket::request::{ self ,Request,  FromRequest};
use rocket::{Outcome, State} ;
use rocket::http::Status;


#[derive(Debug , Serialize , Deserialize)]
pub struct Auth {
    email: String ,
    user_name:String,
    is_admin: bool,
    exp:usize
}

impl From<Auth> for ValidUser {

    fn from(claims:Auth)->Self{

        ValidUser{
            email:claims.email,
            user_name:claims.user_name,
        }
    }      
    
}

impl From<Auth> for AdminUser {

    fn from(claims:Auth)->Self{

        AdminUser{
            email:claims.email,
            user_name:claims.user_name,
            is_admin:true
        }
    }      
    
}


impl Auth {
    
    fn with_email_name ( email : &str, user_name:&str, admin:bool) ->Self {

        Auth {
            email:email.into(),
            user_name:user_name.into(),
            is_admin:admin,
            exp:(Local::now()+Duration::hours(24)).timestamp() as usize
        }
    }

}

pub fn create_token ( email:&str , user_name:&str, is_admin:bool) -> Result<RegisterUser, String>{
     let claims=Auth::with_email_name(email,user_name, is_admin);
     encode(&Header::default(), &claims, get_secret())
            .map(|tk|RegisterUser{
                email:claims.email,
                user_name:claims.user_name,
                token:tk,
                expired_at:NaiveDateTime::from_timestamp(claims.exp as i64,0)
            })
            .map_err(|e| e.to_string())
}

pub fn decode_token (token:&str) ->Result<Auth, String>{

    decode::<Auth>(token, get_secret(),&Validation::default())
            .map(|c| c.claims)
            .map_err(|e| e.to_string())
}

fn get_secret<'a>()->&'a [u8] {    
    dotenv!("JWT_SECRET").as_bytes()
}

fn get_token_prefix<'a> () -> &'a str {
    dotenv!("TOKEN_PREFIX")
}

impl<'a, 'r> FromRequest<'a, 'r> for ValidUser {
    type Error = ();

    /// Extract Auth token from the "Authorization" header.  
    fn from_request(request: &'a Request<'r>) -> request::Outcome<ValidUser, Self::Error> {
        
        if let Some(auth) = extract_auth_from_request(request, get_secret()) {
            Outcome::Success(auth.into())
        } else {
            Outcome::Failure((Status::Forbidden, ()))
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AdminUser {
    type Error = ();

    /// Extract Auth token from the "Authorization" header.  
    fn from_request(request: &'a Request<'r>) -> request::Outcome<AdminUser, Self::Error> {
     
        if let Some(auth) = extract_auth_from_request(request, get_secret()) {
            if (auth.is_admin) {
            Outcome::Success(auth.into())
            }else   {
                Outcome::Failure((Status::Forbidden, ()))
            }
        } else {
            Outcome::Failure((Status::Forbidden, ()))
        }
    }
}

fn extract_auth_from_request(request :&Request , secret: &[u8])->Option<Auth>{
    request.headers()
           .get_one("Authorization")
           .and_then(extract_auth_token_from_header)
           .and_then(|token| decode_token(token).ok())
}

fn extract_auth_token_from_header(header :&str) ->Option<&str>{
    
    let prefix =get_token_prefix();
    // println!("prefix is {}",prefix);
    // println!("header is {}",header);
    if header.starts_with(prefix) {
       Some(&header[prefix.len()..])
    }else{
        None
    }
}
