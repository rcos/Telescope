
// Pagination system inspired by
// https://github.com/diesel-rs/diesel/blob/master/examples/postgres/advanced-blog-cli/src/pagination.rs

use diesel::{
    query_builder::{
        AstPass,
        Query,
        QueryFragment
    },
    sql_types::BigInt,
    PgConnection,
    RunQueryDsl,
    QueryResult,
    pg::Pg,
};
use juniper::GraphQLType;
use diesel::query_builder::QueryId;

/// Trait for paginating diesel queries.
pub trait Paginate: Sized {
    fn paginate(self, offset: i64, count: i64) -> Paginated<Self>;
}

impl <T> Paginate for T {
    fn paginate(self, offset: i64, count: i64) -> Paginated<T> {
        Paginated {
            query: self,
            offset, count
        }
    }
}

/// A paginated query
#[derive(Debug, Copy, Clone, QueryId)]
pub struct Paginated<T: QueryId> {
    query: T,
    offset: i64,
    count: i64,
}

impl <T: QueryId> Paginated<T> {
    pub fn load_and_count<U>(self, conn: &PgConnection) -> QueryResult<PaginatedResults<i64, U>> {
        let results = self.load::<(U, i64)>(conn)?;
        let total = results.first()
            .map(|(_, c)| *c)
            .unwrap_or(0);
        Ok(PaginatedResults {
            total,
            count: self.count,
            offset: self.offset,
            data: results.into_iter().map(|(d, _)| d).collect()
        })
    }
}

impl<T: QueryId + Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T: QueryId> RunQueryDsl<PgConnection> for Paginated<T> {}

impl<T> QueryFragment<Pg> for Paginated<T>
    where T: QueryFragment<Pg>,
{
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.count)?;
        out.push_sql(" OFFSET ");
        out.push_bind_param::<BigInt, _>(&self.offset)?;
        Ok(())
    }
}

/// Pagination struct for use with graphql queries.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct PaginationInput {
    offset: i32,
    count: i32,
}

/// GraphQL Pagination result.
///
/// We cannot derive the GraphQLObject trait here (due to limitations of juniper)
/// so we must derive it for every individual type we use it in.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaginatedResults<N, T> {
    offset: N,
    count: N,
    total: N,
    data: Vec<T>
}
