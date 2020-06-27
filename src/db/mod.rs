use r2d2::PooledConnection;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

pub mod model;
pub mod actions;

/// Database connection type
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;