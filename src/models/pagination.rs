// Pagination system inspired by
// https://github.com/diesel-rs/diesel/blob/master/examples/postgres/advanced-blog-cli/src/pagination.rs

use diesel::{
    pg::Pg,
    query_builder::{AstPass, Query, QueryFragment, QueryId},
    query_dsl::LoadQuery,
    r2d2::{ConnectionManager, Pool},
    result::Error as DieselErr,
    sql_types::BigInt,
    PgConnection, QueryResult, RunQueryDsl,
};

use actix_web::{rt::blocking::BlockingError, web::block};

use crate::{models::users::User, web::api::graphql::ApiContext};

/// Trait for paginating diesel queries.
pub trait Paginate: Sized + QueryId {
    fn paginate(self, offset: i64, count: i64) -> Paginated<Self>;
    fn offset(self, offset: i64) -> Paginated<Self>;
}

impl<T: QueryId> Paginate for T {
    fn paginate(self, offset: i64, count: i64) -> Paginated<T> {
        Paginated {
            query: self,
            offset,
            count: Some(count),
        }
    }

    fn offset(self, offset: i64) -> Paginated<T> {
        Paginated {
            query: self,
            offset,
            count: None,
        }
    }
}

/// A paginated query
#[derive(Debug, Copy, Clone, QueryId)]
pub struct Paginated<T> {
    query: T,
    offset: i64,
    /// If none, get all remaining.
    count: Option<i64>,
}

/// GraphQL Pagination result.
///
/// We cannot derive the GraphQLObject trait here (due to limitations of juniper)
/// so we must derive it for every individual type we use it in.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaginatedData<N, T> {
    offset: N,
    count: N,
    total: N,
    data: Vec<T>,
}

/// Return type from pagination database calls.
pub type PaginatedResult<N, T, E> = Result<PaginatedData<N, T>, E>;

impl<T> Paginated<T> {
    /// Load and count the data from a query.
    pub fn load_and_count<U>(self, conn: &PgConnection) -> PaginatedResult<i64, U, DieselErr>
    where
        Self: LoadQuery<PgConnection, (U, i64)>,
    {
        let offset = self.offset;
        let results = self.load::<(U, i64)>(conn)?;
        let total = results.first().map(|(_, c)| *c).unwrap_or(0);
        Ok(PaginatedData {
            total,
            count: results.len() as i64,
            offset,
            data: results.into_iter().map(|(d, _)| d).collect(),
        })
    }

    /// Async version of previous function.
    pub async fn async_load_and_count<U>(
        self,
        pooled_conn: &Pool<ConnectionManager<PgConnection>>,
    ) -> PaginatedResult<i64, U, BlockingError<DieselErr>>
    where
        Self: LoadQuery<PgConnection, (U, i64)>,
        U: Send + 'static,
        T: Send + 'static,
    {
        let pool = pooled_conn.clone();
        block(move || {
            let conn = pool
                .get()
                .map_err(|e| {
                    error!("Could not get Database Connection from Pool: {}", e);
                })
                .unwrap();
            self.load_and_count(&conn)
        })
        .await
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T: QueryId> RunQueryDsl<PgConnection> for Paginated<T> {}

impl<T: QueryId> QueryFragment<Pg> for Paginated<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t");
        if let Some(limit) = self.count {
            out.push_sql(" LIMIT ");
            out.push_bind_param::<BigInt, _>(&limit)?;
        }
        out.push_sql(" OFFSET ");
        out.push_bind_param::<BigInt, _>(&self.offset)?;
        Ok(())
    }
}

impl<N, T> PaginatedData<N, T> {
    /// Change the numeric type associated with the paginated data using
    /// the standard conversion function.
    pub fn change_numeric<O: From<N>>(self) -> PaginatedData<O, T> {
        PaginatedData {
            offset: self.offset.into(),
            count: self.count.into(),
            total: self.total.into(),
            data: self.data,
        }
    }
}

/// Pagination input represented by the offset into the dataset and the maximum
/// number of entries to fetch.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, juniper::GraphQLInputObject, Default)]
pub struct PaginationInput {
    /// The offset into the dataset.
    offset: i32,
    /// The maximum number of entries to fetch.
    /// If this is null then the number of fetched entries will be the total
    /// number of entries minus the offset.
    count: Option<i32>,
}

macro_rules! impl_juniper_pagination {
    ($t:ty, $n:literal) => {
        #[graphql_object(
                                                    Context = ApiContext,
                                                    name = $n
                                                )]
        impl PaginatedData<i32, $t> {
            /// The offset into the dataset.
            fn offset(&self) -> i32 {
                self.offset
            }

            /// The number of items retrieved.
            fn count(&self) -> i32 {
                self.count
            }

            /// The total number of items in the dataset.
            fn total(&self) -> i32 {
                self.total
            }

            /// The data retrieved.
            fn data(&self) -> &Vec<$t> {
                &self.data
            }
        }
    };
}

impl_juniper_pagination!(User, "PaginatedUsers");
