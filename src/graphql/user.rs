use juniper::{GraphQLInputObject};
use crate::graphql::root::Context;
use crate::repository::user::User as UserModel;

pub struct User {
    id: i32,
    name: String,
    email: String,
    created_at: String,
    updated_at: String,
}

#[juniper::graphql_object(Context = Context)]
impl User {
    fn id(&self) -> &i32 {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn email(&self) -> &str {
        &self.email
    }

    fn createdAt(&self) -> &str {
        &self.created_at
    }

    fn updatedAt(&self) -> &str {
        &self.updated_at
    }
}

impl From<UserModel> for User {
    fn from(user: UserModel) -> Self {
        User { 
            id: user.id,
            name: user.name,
            email: user.email,
            created_at: user.created_at.unwrap().format("%Y-%m-%d %T").to_string(),
            updated_at: user.updated_at.unwrap().format("%Y-%m-%d %T").to_string(),
         }
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "New thermostat status")]
pub struct NewUser {
    name: String,
    email: String,
    password: String,
}
