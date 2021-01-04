use juniper::GraphQLInputObject;
use serde::Deserialize;
use validator::{Validate, ValidationError};

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
        let created_at: String;
        let updated_at: String;
        if user.created_at.is_some() {
            created_at = user.created_at.unwrap().format("%Y-%m-%d %T").to_string();
        } else {
            created_at = "".to_string();
        }
        if user.updated_at.is_some() {
            updated_at = user.updated_at.unwrap().format("%Y-%m-%d %T").to_string();
        } else {
            updated_at = "".to_string();
        }
        User {
            id: user.id,
            name: user.name,
            email: user.email,
            created_at: created_at,
            updated_at: updated_at,
        }
    }
}

#[graphql(description = "New user")]
#[derive(GraphQLInputObject, Validate, Deserialize)]
pub struct NewUser {
    #[validate(length(min = 2))]
    pub name: String,
    #[validate(email, custom = "validate_unique_email")]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

fn validate_unique_email(email: &str) -> Result<(), ValidationError> {
    if email == "email" {
        // the value of the username will automatically be added later
        return Err(ValidationError::new("terrible_username"));
    }

    Ok(())
}

