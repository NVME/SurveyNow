# SurveyNow
Survey application  RUST version


git pull origin master --allow-unrelated-histories

https://dev.to/werner/practical-rust-web-development-api-rest-29g1

https://dzone.com/articles/creating-a-rest-api-in-rust-using-rocket-and-diese  ***

https://api.rocket.rs/v0.4/rocket_contrib/databases/index.html

https://rocket.rs/v0.4/guide/state/#databases 

http://diesel.rs/guides/getting-started/

https://docs.diesel.rs/diesel/dsl/index.html

http://docs.diesel.rs/diesel/index.html

https://docs.diesel.rs/diesel/query_dsl/trait.RunQueryDsl.html

https://rocket.rs/v0.4/guide/requests/

https://api.rocket.rs/v0.4/rocket/derive.Responder.html

https://api.rocket.rs/v0.4/rocket/request/trait.FromRequest.html

https://api.rocket.rs/v0.4/rocket/response/trait.Responder.html

https://api.rocket.rs/v0.4/rocket/http/struct.Status.html

https://rocket.rs/v0.4/guide/responses/

https://api.rocket.rs/v0.4/rocket_contrib/databases/index.html

https://github.com/SergioBenitez/Rocket/tree/master/examples

https://rocket.rs/v0.4/guide/configuration/

https://lib.rs/crates/chrono

https://docs.diesel.rs/diesel/macro.table.html

http://docs.diesel.rs/diesel/deserialize/trait.Queryable.html

https://github.com/Keats/jsonwebtoken

When this trait is derived, it will assume that the order of fields on your struct match the order of the fields in the query. This means that field order is significant if you are using #[derive(Queryable)]. Field name has no affect.

rustup update && cargo update

sudo apt install libpq-dev

cargo install diesel_cli --no-default-features --features postgres chrono

echo DATABASE_URL=postgres://postgres:@localhost/surveys > .env

diesel setup

diesel migration generate create_users

sudo kill -9 $(sudo lsof -t -i:8000)

https://github.com/passcod/cargo-watch

$ sudo ROCKET_ENV=dev cargo run