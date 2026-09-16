use std::{env, str};
use chrono::{Duration, Utc};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use crate::db;
use crate::services::user_service::UserService;
use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::Request;
pub struct AuthenticatedUser {
    pub user_id: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize
}

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

#[derive(Debug, Serialize)]
pub struct  LoginResponse {
    pub token: String
}

pub fn generate_token(user_id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(1))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(secret.as_bytes())
    )
}

#[post("/login", format = "json", data = "<data>")]
pub fn login(data: Json<LoginRequest>, connection: db::Connection) -> Result<Json<LoginResponse>, Status> {
    let mut connection = connection.0;

    UserService::login(&mut connection, &data.password, &data.email)
        .map_err(|error| error.status())
        .and_then(|user| {
            let token = generate_token(user.id)
                .map_err(|_| Status::InternalServerError)?;

            Ok(Json(LoginResponse {
                token
            }))
        })
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

#[get("/profile")]
pub fn profile(user: AuthenticatedUser, connection : db::Connection) -> Result<Json<UserResponse>, Status> {
    let mut connection = connection.0;

    UserService::get_by_id(&mut connection, user.user_id)
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

#[put("/profile", format = "json", data = "<data>")]
pub fn update_profile(user: AuthenticatedUser, connection: db::Connection, data: Json<UpdateUserRequest>) -> Result<Json<UserResponse>, Status>{
    let mut connection = connection.0;

    UserService::update(
        &mut connection,
        user.user_id,
        data.username.as_deref(),
        data.email.as_deref()
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

#[rocket::async_trait]
impl <'r> FromRequest<'r> for AuthenticatedUser  {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let authorization = request.headers().get_one("Authorization");

        let authorization = match authorization {
            Some(value) => value,
            None => return Outcome::Error((Status::Unauthorized, ())),
        };

        let token = match authorization.strip_prefix("Bearer ") {
            Some(token) => token,
            None => return Outcome::Error((Status::Unauthorized, ())),
        };

        let secret = match env::var("JWT_SECRET") {
            Ok(secret) => secret,
            Err(_) => {
                return Outcome::Error((Status::InternalServerError, ()))
            }
        };

        let validation = jsonwebtoken::Validation::default();

        let token_data = match jsonwebtoken::decode::<Claims> (
            token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
            &validation
        ){
            Ok(data) => data,
            Err(_) =>  {
                return Outcome::Error((Status::Unauthorized, ()))
            }
        };

        Outcome::Success(AuthenticatedUser {
            user_id: token_data.claims.sub
        })
    }
}