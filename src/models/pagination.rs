
// Pagination system inspired by
// https://github.com/diesel-rs/diesel/blob/master/examples/postgres/advanced-blog-cli/src/pagination.rs

use diesel::{
    query_builder::{
        AstPass,
        Query,
        QueryFragment,
        QueryId,
    },
    r2d2::{
        Pool,
        ConnectionManager
    },
    query_dsl::LoadQuery,
    result::Error as DieselErr,
    sql_types::BigInt,
    PgConnection,
    RunQueryDsl,
    QueryResult,
    pg::Pg,
};

use juniper::{GraphQLType, ScalarValue, ScalarRefValue, Executor, Selection, Value, Arguments, ExecutionResult, Registry, meta::MetaType, FieldError, DefaultScalarValue};

use actix_web::{
    web::block,
    rt::blocking::BlockingError
};

use crate::web::api::graphql::ApiContext;
use std::any;

/// Trait for paginating diesel queries.
pub trait Paginate: Sized + QueryId {
    fn paginate(self, offset: i64, count: i64) -> Paginated<Self>;
}

impl <T: QueryId> Paginate for T {
    fn paginate(self, offset: i64, count: i64) -> Paginated<T> {
        Paginated {
            query: self,
            offset, count
        }
    }
}

/// A paginated query
#[derive(Debug, Copy, Clone, QueryId)]
pub struct Paginated<T> {
    query: T,
    offset: i64,
    count: i64,
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
    data: Vec<T>
}

/// Return type from pagination database calls.
pub type PaginatedResult<N, T, E> = Result<PaginatedData<N, T>, E>;

impl <T> Paginated<T> {
    /// Load and count the data from a query.
    pub fn load_and_count<U>(self, conn: &PgConnection) -> PaginatedResult<i64, U, DieselErr>
    where Self: LoadQuery<PgConnection, (U, i64)> {
        let count = self.count;
        let offset = self.offset;
        let results = self.load::<(U, i64)>(conn)?;
        let total = results.first()
            .map(|(_, c)| *c)
            .unwrap_or(0);
        Ok(PaginatedData {
            total,
            count,
            offset,
            data: results.into_iter().map(|(d, _)| d).collect()
        })
    }

    /// Async version of previous function.
    pub async fn async_load_and_count<U>(self, pooled_conn: &Pool<ConnectionManager<PgConnection>>)
        -> PaginatedResult<i64, U, BlockingError<DieselErr>>
    where
        Self: LoadQuery<PgConnection, (U, i64)>,
        U: Send + 'static,
        T: Send + 'static
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
        }).await
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T: QueryId> RunQueryDsl<PgConnection> for Paginated<T> {}

impl<T: QueryId> QueryFragment<Pg> for Paginated<T>
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

impl<N, T> PaginatedData<N, T> {
    /// Change the numeric type associated with the paginated data using
    /// the standard conversion function.
    pub fn change_numeric<O: From<N>>(self) -> PaginatedData<O, T> {
        PaginatedData {
            offset: self.offset.into(),
            count: self.count.into(),
            total: self.total.into(),
            data: self.data
        }
    }
}

/// Pagination struct for use with graphql queries.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct PaginationInput {
    offset: i32,
    count: i32,
}

/// Implementation of Juniper GraphQL type on paginated data.
impl<T> GraphQLType for PaginatedData<i32, T>
where
    T: GraphQLType<Context = ApiContext, TypeInfo = ()>,
{
    type Context = ApiContext;
    type TypeInfo = ();

    fn name(_: &Self::TypeInfo) -> Option<&str> {
        None
    }

    fn meta<'r>(_: &(), registry: &mut Registry<'r>) -> MetaType<'r, DefaultScalarValue>
    where DefaultScalarValue: 'r,
    {
        let fields = &[
            registry.field::<&i32>("offset", &()),
            registry.field::<&i32>("count", &()),
            registry.field::<&i32>("total", &()),
            registry.field::<&Vec<T>>("data", &())
        ];

        registry
            .build_object_type::<PaginatedData<i32, T>>(&(), fields)
            .into_meta()
    }

    fn resolve_field(
        &self,
        info: &(),
        field_name: &str,
        arguments: &Arguments<DefaultScalarValue>,
        executor: &Executor<Self::Context, DefaultScalarValue>
    ) -> ExecutionResult<DefaultScalarValue> {
        match field_name {
            "offset"    => executor.resolve_with_ctx(info, &self.offset),
            "count"     => executor.resolve_with_ctx(info, &self.count),
            "total"     => executor.resolve_with_ctx(info, &self.total),
            "data"      => executor.resolve_with_ctx(info, &self.data),
            other => panic!("No field named {} found on PaginationData.", other)
        }
    }

    fn resolve_into_type(
        &self,
        _: &(),
        type_name: &str,
        selection_set: Option<&[Selection<DefaultScalarValue>]>,
        executor: &Executor<Self::Context, DefaultScalarValue>
    ) -> ExecutionResult<DefaultScalarValue> {
        unimplemented!()
    }

    fn concrete_type_name(
        &self,
        context: &Self::Context,
        _: &()
    ) -> String {
        format!("Paginated{}",)
    }

    fn resolve(
        &self,
        _: &(),
        selection_set: Option<&[Selection<DefaultScalarValue>]>,
        executor: &Executor<Self::Context, DefaultScalarValue>
    ) -> Value<DefaultScalarValue> {
        unimplemented!()
    }
}
