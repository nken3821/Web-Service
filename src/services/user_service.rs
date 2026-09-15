use bcrypt::{hash, DEFAULT_COST, verify};
use diesel::MysqlConnection;
use crate::models::user::{NewUser, UpdateUser, User};
use crate::repository::user_repository::UserRepository;
use crate::error::AppError;

pub struct UserService;

impl UserService {

    pub fn login(connection : &mut MysqlConnection, password: &str, email: &str) -> Result<User, AppError> {
        let user = UserRepository::find_by_email(email.to_string(), connection)
            .map_err(|error| match error {
                diesel::result::Error::NotFound => AppError::UserNotFound,
                _ => AppError::DatabaseError,
            })?;

        let valid = verify(password, &user.password_hash)
            .map_err(|_| AppError::AuthenticationError)?;

        if !valid {
            return Err(AppError::AuthenticationError);
        }

        Ok(user)
    }
    pub fn register(connection : &mut MysqlConnection, username: &str, email: &str, password: &str) -> Result<User, AppError> {

        if username.trim().is_empty() || email.trim().is_empty() || password.trim().is_empty() {
            return Err(AppError::ValidationError)
        }

        if password.len() < 6 {
            return Err(AppError::ValidationError)
        }

        match UserRepository::find_by_email(email.to_string(), connection) {
            Ok(_) => {
                return Err(AppError::UserAlreadyExists)
            }
            Err(diesel::result::Error::NotFound) => {

            }
            Err(_) => {
                return Err(AppError::DatabaseError)
            }
        }

        let password_hash = hash(password, DEFAULT_COST)
            .map_err(|_| AppError::PasswordHashError)?;

        let new_user = NewUser {
            username: username.to_string(),
            email: email.to_string(),
            password_hash,
        };

        UserRepository::create(new_user, connection)
            .map_err(|error| {
            println!("CREATE USER DATABASE ERROR: {:?}", error);
            AppError::DatabaseError
        })
    }

    pub fn get_all(connection :&mut MysqlConnection) -> Result<Vec<User>, AppError> {
        UserRepository::find_all(connection)
            .map_err(|_| AppError::DatabaseError)
    }

    pub fn get_by_id(connection: &mut MysqlConnection, user_id: i32) -> Result<User, AppError> {
        match UserRepository::find_by_id(user_id, connection) {
            Ok(user) => Ok(user),

            Err(diesel::result::Error::NotFound) => {
                Err(AppError::UserNotFound)
            },

            Err(_) => {
                Err(AppError::DatabaseError)
            }
        }
    }

    pub fn delete(connection: &mut MysqlConnection, user_id: i32) -> Result<(), AppError> {
        let affected_rows = UserRepository::delete(connection, user_id)
            .map_err(|_| AppError::DatabaseError)?;

        if affected_rows == 0 {
            return Err(AppError::UserNotFound);
        }

        Ok(())
    }

    pub fn update(connection: &mut MysqlConnection, user_id: i32, username: Option<&str>, email: Option<&str>) -> Result<User, AppError> {
        if username.is_none() && email.is_none() {
            return Err(AppError::ValidationError);
        }
        
        if let Some(username) = username {
            if username.trim().is_empty() {
                return Err(AppError::ValidationError);
            }
        }

        if let Some(email) = email {
            if email.trim().is_empty() {
                return Err(AppError::ValidationError);
            }
        }

        UserRepository::find_by_id(user_id, connection)
            .map_err(|error| match error {
                diesel::result::Error::NotFound => AppError::UserNotFound,
                _ => AppError::DatabaseError,
            })?;

        let changes = UpdateUser {
            username : username.map(|value| value.to_string()),
            email: email.map(|value| value.to_string())
        };

        UserRepository::update(connection, user_id, changes)
            .map_err(|_| AppError::DatabaseError )?;

        UserRepository::find_by_id(user_id, connection)
            .map_err(|_| AppError::DatabaseError)
    }
}