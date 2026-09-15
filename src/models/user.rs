use chrono::NaiveDateTime;
use diesel;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use crate::schema::users;

#[derive(Selectable, Queryable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password_hash: String,
    pub email: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>
}

