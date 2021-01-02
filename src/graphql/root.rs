use juniper::{graphql_object, EmptySubscription, RootNode};

pub struct Context {}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[derive(Clone, Debug)]
struct User {
    id: i32,
}

#[graphql_object(context = Context)]
impl User {
    fn id(&self) -> i32 {
        self.id
    }
}

#[graphql_object(Context = Context)]
impl QueryRoot {
    async fn user() -> Vec<User> {
        vec![User { id: 1 }]
    }
}

pub struct MutationRoot;

#[graphql_object(Context = Context)]
impl MutationRoot {
    async fn create_user() -> Vec<User> {
        vec![User { id: 1 }]
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::<Context>::new())
}
