table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        #[sql_name = "username"]
        user_name -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        #[sql_name = "isadmin"]
        is_admin -> Bool,
    }
}
