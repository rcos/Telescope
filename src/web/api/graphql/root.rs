use juniper::{FieldError, FieldResult, RootNode, Value};

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

use crate::{
    models::{Email, PasswordRequirements, User},
    web::{DbConnection, RequestContext},
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
    pub identity: Uuid,
}

impl ApiContext {
    /// Try to make a new API context. Return none if not logged in.
    pub async fn new(
        connection_pool: Pool<ConnectionManager<PgConnection>>,
        parent: &RequestContext,
    ) -> Option<Self> {
        let id = parent
            .identity()
            .identity()
            .and_then(|id| Uuid::parse_str(&id).ok());

        if let Some(uuid) = id {
            let conn = parent.get_db_connection().await;
            User::get_from_db_by_id(conn, uuid).await.map(|user| Self {
                connection_pool,
                schema: Self::make_schema(),
                identity: user.id,
            })
        } else {
            None
        }
    }

    /// Get the GraphQL schema object.
    pub fn make_schema() -> Schema {
        Schema::new(QueryRoot, MutationRoot)
    }

    /// Get a database connection. Log any errors and then map to a juniper error type.
    pub fn get_db_conn(&self) -> FieldResult<DbConnection> {
        self.connection_pool.get().map_err(|e| {
            error!("Could not get database connection: {}", e);
            FieldError::new(e, Value::null())
        })
    }
}

impl juniper::Context for ApiContext {}

/// The root of all graphql queries.
pub struct QueryRoot;

/// The root of all graphql mutations.
pub struct MutationRoot;

#[juniper::object(Context = ApiContext)]
impl QueryRoot {
    /// Telescope Version
    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    /// Get the currently logged in user.
    fn me(ctx: &ApiContext) -> FieldResult<User> {
        use crate::schema::users::dsl::*;

        let uid = ctx.identity;
        let mut conn = ctx.get_db_conn()?;

        users.find(uid).first(&conn).map_err(|e| {
            error!("Could not find user for id {}: {}", uid, e);
            FieldError::new("User not found.", Value::null())
        })
    }

    /// Get a list of all users.
    fn users(ctx: &ApiContext) -> FieldResult<Vec<User>> {
        use crate::schema::users::dsl::*;
        let mut conn = ctx.get_db_conn()?;
        users.load(&conn).map_err(|e| {
            error!("Could not load users from database.");
            FieldError::new(e, Value::null())
        })
    }

    /// Get a list of all emails
    pub fn emails(ctx: &ApiContext) -> FieldResult<Vec<Email>> {
        use crate::schema::emails::dsl::*;
        let conn = ctx.get_db_conn()?;
        emails.filter(is_visible).load(&conn).map_err(|e| {
            error!("Could not load emails from database");
            FieldError::new(e, Value::null())
        })
    }

    /// Check if a password satisfies password validity requirements.
    pub fn password_requirements(password: String) -> PasswordRequirements {
        PasswordRequirements::for_password(&password)
    }
}

#[juniper::object(Context = ApiContext)]
impl MutationRoot {}
