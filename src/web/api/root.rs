use juniper::{FieldError, FieldResult, RootNode, Value};

use super::User;
use crate::schema::users::dsl::users;
use crate::web::api::{Email, PasswordRequirements};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use crate::web::RequestContext;

/// GraphQL Schema type. Used for executing all GraphQL requests.
pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

/// Context accessible to juniper when resolving GraphQl API requests.
pub struct ApiContext {
    /// Database connection pool.
    pub connection_pool: Pool<ConnectionManager<PgConnection>>,
    /// Schema object to execute GraphQl queries.
    pub schema: Schema,
    /// Identity object to do authentication
    pub identity: Option<String>
}

impl ApiContext {
    /// Get the GraphQL schema object.
    pub fn get_schema() -> Schema {
        Schema::new(QueryRoot, MutationRoot)
    }
}

/// The root of all graphql queries.
pub struct QueryRoot;

/// The root of all graphql mutations.
pub struct MutationRoot;

#[juniper::object(Context = ApiContext)]
impl QueryRoot {
    #[graphql(description = "List of all users.")]
    pub fn users(ctx: &ApiContext) -> FieldResult<Vec<User>> {
        let mut conn = ctx.connection_pool.get().map_err(|e| {
            error!("Could not get database connection.");
            FieldError::new(e, Value::null())
        })?;
        users.load(&conn).map_err(|e| {
            error!("Could not load users from database.");
            FieldError::new(e, Value::null())
        })
    }

    #[graphql(description = "List of user emails.")]
    pub fn emails(ctx: &ApiContext) -> FieldResult<Vec<Email>> {
        unimplemented!()
    }

    #[graphql(description = "Checks if a password is valid.")]
    pub fn password_requirements(password: String) -> PasswordRequirements {
        PasswordRequirements::for_password(&password)
    }
}

#[juniper::object(Context = ApiContext)]
impl MutationRoot {}
