use diesel::prelude::*;
use diesel;
use diesel::sql_types::{BigInt, Unsigned};
use crate::models::user::{User, NewUser, UpdateUser};
use crate::schema::users;

#[derive(QueryableByName)]
struct LastInsertId {
    #[diesel(sql_type = Unsigned<BigInt>)]
    id: u64
}
pub struct UserRepository;

impl UserRepository {
    pub fn find_by_id(user_id: i32, connection: &mut MysqlConnection) -> QueryResult<User> {
        users::table
            .find(user_id)
            .select(User::as_select())
            .first(connection)
    }

    pub fn find_by_email(user_email: String, connection: &mut MysqlConnection) -> QueryResult<User> {
        users::table
            .filter(users::email.eq(user_email))
            .select(User::as_select())
            .first(connection)
    }

    pub fn create(new_user: NewUser, connection: &mut MysqlConnection) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(connection)?;

        let result = diesel::sql_query(
            "SELECT LAST_INSERT_ID() AS id"
        ).get_result::<LastInsertId>(connection)?;

        users::table
            .find(result.id as i32)
            .select(User::as_select())
            .first(connection)
    }

    pub fn find_all(connection : &mut MysqlConnection) -> QueryResult<Vec<User>> {
        users::table
            .select(User::as_select())
            .load(connection)
    }

    pub fn delete(connection : &mut MysqlConnection, user_id: i32) -> QueryResult<usize> {
        diesel::delete(users::table.find(user_id))
            .execute(connection)
    }

    pub fn update(connection: &mut MysqlConnection, user_id: i32, changes: UpdateUser) -> QueryResult<usize> {
        diesel::update(users::table.find(user_id))
            .set(&changes)
            .execute(connection)
    }
}

