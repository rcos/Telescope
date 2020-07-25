use juniper::RootNode;

use diesel::{
    r2d2::{
        Pool,
        ConnectionManager
    },
    PgConnection,
};

/// GraphQL Schema type. Used for executing all GraphQL requests.
pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

/// Context accessible to juniper when resolving GraphQl API requests.
pub struct ApiContext {
    /// Database connection pool.
    pub connection_pool: Pool<ConnectionManager<PgConnection>>,
    /// Schema object to execute GraphQl queries.
    pub schema: Schema
}

impl ApiContext {
    /// Create a new API context object.
    pub fn new(pool: &Pool<ConnectionManager<PgConnection>>) -> Self {
        ApiContext {
            schema: Schema::new(QueryRoot, MutationRoot),
            connection_pool: pool.clone()
        }
    }
}


/// The root of all graphql queries.
pub struct QueryRoot;

/// The root of all graphql mutations.
pub struct MutationRoot;


#[juniper::object(Context = ApiContext)]
impl QueryRoot {

}

#[juniper::object(Context = ApiContext)]
impl MutationRoot {

}