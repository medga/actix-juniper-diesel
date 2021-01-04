use crate::schema::users;
use crate::graphql::user::NewUser as NewUserGraph;
use chrono::NaiveDateTime;
use diesel::prelude::*;

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
impl From<NewUserGraph> for NewUser {
    fn from(user: NewUserGraph) -> Self {
        NewUser {
            name: user.name,
            email: user.email,
            password: user.password
        }
    }
}

impl User {
    pub fn check_email_unique(connection: &PgConnection, _email: String) -> bool {
        use crate::schema::users::dsl::*;
        use diesel::dsl::*;

        users.filter(email.eq(_email)).select(count_star()).first(connection) == Ok(0)
    }
    pub fn get_user(connection: &PgConnection) -> QueryResult<User> {
        use crate::schema::users::dsl::*;

        users.limit(1).get_result(connection)
    }

    pub fn get_users(connection: &PgConnection) -> QueryResult<Vec<User>> {
        use crate::schema::users::dsl::*;

        users.limit(10).load::<User>(connection)
    }

    pub fn insert(connection: &PgConnection, data: NewUser) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(&data)
            .execute(connection).expect("Can not create user!");

        User::get_last(connection)
    }

    fn get_last(connection: &PgConnection) -> QueryResult<User> {
        use crate::schema::users::dsl::*;

        users.order(id.desc()).limit(1).get_result(connection)
    }
}
