use rocket::http::Status;

#[derive(Debug)]
pub enum AppError {
    UserAlreadyExists,
    UserNotFound,
    ValidationError,
    AuthenticationError,
    DatabaseError,
    PasswordHashError,
}

impl AppError  {
    pub fn status(&self) -> Status {
        match self {
            AppError::UserNotFound => Status::NotFound,
            AppError::UserAlreadyExists => Status::Conflict,
            AppError::ValidationError => Status::BadRequest,
            AppError::AuthenticationError => Status::Unauthorized,
            AppError::DatabaseError => Status::InternalServerError,
            AppError::PasswordHashError => Status::InternalServerError,
        }
    }
}