#[macro_use] extern crate rocket;

use crate::routes::auth::{delete_user, get_user, get_users, login, register, update_user};

mod models;
mod db;
mod schema;
mod routes;
mod repository;
mod services;
mod error;

#[rocket::main]
async fn main() {
    rocket::build()
        .manage(db::connect())
        .mount("/auth", routes![register, login])
        .mount("/", routes![get_users, get_user, delete_user, update_user])
        .launch().await.unwrap();
}
