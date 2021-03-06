use juniper::{EmptySubscription, FieldResult, RootNode};
use std::sync::Arc;

use crate::db::DbPool;
use crate::repository::user::*;
use crate::graphql::user::{User as UserType,NewUser as UserInput };

pub struct Context {
    pub conn: Arc<DbPool>,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::graphql_object(context = Context)]
impl QueryRoot {
    #[graphql(description = "Get the user by id")]
    async fn user(context: &Context) -> FieldResult<UserType> {
        let connection = &context.conn.get()?;
        let user = User::get_user(connection)?;

        Ok(UserType::from(user))
    }

    #[graphql(description = "List of all users")]
    fn users(context: &Context) -> FieldResult<Vec<UserType>> {
        let connection = &context.conn.get()?;
        let users = User::get_users(connection)?;
        let retult = users.into_iter().map(|user| {
            UserType::from(user)
        }).collect();
        
        Ok(retult)
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(context = Context)]
impl MutationRoot {
    async fn create_user(context: &Context, data: UserInput) -> FieldResult<UserType> {
        let connection = &context.conn.get()?;
        let user = User::insert(connection, NewUser::from(data))?;
        
        Ok(UserType::from(user))
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
