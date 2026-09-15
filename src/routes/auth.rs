use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use crate::db;
use crate::services::user_service::UserService;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
}

#[post("/login", format = "json", data = "<data>")]
pub fn login(data: Json<LoginRequest>, connection: db::Connection) -> Result<Json<UserResponse>, Status> {
    let mut connection = connection.0;

    UserService::login(&mut connection, &data.password, &data.email)
        .map(|user| {
            Json(UserResponse {
                id: user.id,
                username: user.username,
                email: user.email,
                created_at: user.created_at.to_string(),
                updated_at: user.updated_at.to_string(),
            })
        })
        .map_err(|error| error.status())
}

#[post("/register", format = "json", data = "<data>" )]
pub fn register(data: Json<RegisterRequest>, connection: db::Connection) -> Result<Json<UserResponse>, Status> {
    let mut connection = connection.0;
    println!("REGISTER ROUTE REACHED");
     UserService::register(
        &mut connection,
        &data.username,
        &data.email,
        &data.password
    )
         .map(|user| {
         Json(UserResponse {
             id: user.id,
             username: user.username,
             email: user.email,
             created_at: user.created_at.to_string(),
             updated_at: user.updated_at.to_string(),
         })
     })
         .map_err(|error| error.status())
}

#[get("/users")]
pub fn get_users(connection: db::Connection)-> Result<Json<Vec<UserResponse>>, Status> {
    let mut connection = connection.0;

    UserService::get_all(&mut connection)
        .map(|users| {
            Json(
                users.into_iter()
                    .map(|user| UserResponse {
                        id: user.id,
                        username: user.username,
                        email: user.email,
                        created_at: user.created_at.to_string(),
                        updated_at: user.updated_at.to_string(),
                    })
                    .collect()
            )
        })
        .map_err(|error| error.status())
}

#[get("/users/<id>")]
pub fn get_user(id:i32, connection : db::Connection) -> Result<Json<UserResponse>, Status> {
    let mut connection = connection.0;

    UserService::get_by_id(&mut connection, id)
        .map(|user|  {
            Json(UserResponse {
                id: user.id,
                username: user.username,
                email: user.email,
                created_at: user.created_at.to_string(),
                updated_at: user.updated_at.to_string(),
            })
        })
        .map_err(|error| error.status())
}

#[delete("/users/<id>")]
pub fn delete_user(id:i32, connection : db::Connection) -> Result<Status, Status> {
    let mut connection = connection.0;

    UserService::delete(&mut connection, id)
        .map(|_| Status::NoContent)
        .map_err(|error| error.status())
}

#[put("/users/<id>", format = "json", data = "<data>")]
pub fn update_user(id:i32, connection : db::Connection, data: Json<UpdateUserRequest>) -> Result<Json<UserResponse>, Status> {
    let mut connection = connection.0;


    UserService::update(&mut connection, id, data.username.as_deref(), data.email.as_deref())
        .map(|user| {
            Json(UserResponse {
                id: user.id,
                username: user.username,
                email: user.email,
                created_at: user.created_at.to_string(),
                updated_at: user.updated_at.to_string(),
            })
        })
        .map_err(|error| error.status())
}