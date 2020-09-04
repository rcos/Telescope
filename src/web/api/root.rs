use juniper::{FieldError, FieldResult, RootNode, Value};

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

use crate::{
    models::{
        Email,
        User
    },
    web::{
        RequestContext,
        DbConnection,
        api::PasswordRequirements
    }
};
use uuid::Uuid;

/// GraphQL Schema type. Used for executing all GraphQL requests.
pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

/// Context accessible to juniper when resolving GraphQl API requests.
pub struct ApiContext {
    /// Database connection pool.
    connection_pool: Pool<ConnectionManager<PgConnection>>,
    /// Schema object to execute GraphQl queries.
    pub schema: Schema,
    /// User identity UUID.
    identity: Uuid,
}

impl ApiContext {
    /// Try to make a new API context. Return none if not logged in.
    pub fn new(connection_pool: Pool<ConnectionManager<PgConnection>>, parent: &RequestContext) -> Option<Self> {
        parent.identity()
            .identity()
            .map(|id| Uuid::parse_str(&id).ok())
            .flatten()
            .map(|uuid| Self {
                connection_pool,
                schema: Self::make_schema(),
                identity: uuid
            })
    }

    /// Get the GraphQL schema object.
    pub fn make_schema() -> Schema {
        Schema::new(QueryRoot, MutationRoot)
    }

    /// Get a database connection. Log any errors and then map to a juniper error type.
    pub fn get_db_conn(&self) -> FieldResult<DbConnection> {
        self.connection_pool.get().map_err(|e| {
            error!("Could not get database connecttion: {}", e);
            FieldError::new(e, Value::null())
        })
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
        use crate::schema::users::dsl::*;
        let mut conn = ctx.get_db_conn()?;
        users.load(&conn).map_err(|e| {
            error!("Could not load users from database.");
            FieldError::new(e, Value::null())
        })
    }

    #[graphql(description = "List of user emails.")]
    pub fn emails(ctx: &ApiContext) -> FieldResult<Vec<Email>> {
        use crate::schema::emails::dsl::*;
        let conn = ctx.get_db_conn()?;
        emails
            .filter(is_visible)
            .load(&conn)
            .map_err(|e| {
                error!("Could not load emails from database");
                FieldError::new(e, Value::null())
            })
    }

    #[graphql(description = "Checks if a password is valid.")]
    pub fn password_requirements(password: String) -> PasswordRequirements {
        PasswordRequirements::for_password(&password)
    }
}

#[juniper::object(Context = ApiContext)]
impl MutationRoot {}
