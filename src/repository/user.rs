use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::schema::users;

#[derive(Queryable, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    name: String,
    email: String,
    password: String,
}

impl User {
    pub fn get_user(connection: &PgConnection) -> QueryResult<User> {
        use crate::schema::users::dsl::*;

        users.limit(1).get_result(connection)
    }

    pub fn get_users(connection: &PgConnection) -> QueryResult<Vec<User>> {
        use crate::schema::users::dsl::*;

        users.limit(10).load::<User>(connection)
    }

    pub fn insert(connection: &PgConnection, data: NewUser) -> QueryResult<usize> {
        diesel::insert_into(users::table)
            .values(&data)
            .execute(connection)
    }
}
