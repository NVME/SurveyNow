table! {//https://docs.diesel.rs/diesel/macro.table.html
    users (id) {
        id -> Int4,        
        #[sql_name = "username"]
        user_name -> Varchar,
        email -> Varchar,
        password -> Varchar,        
        created_at -> Timestamp,
    }
}
