use actix::{Actor, AsyncContext, Context};
use chrono::Local;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use std::time;

/// The database janitor actor.
pub struct DbJanitor {
    /// A reference to the database to connection pool to grab connections from
    /// when necessary.
    db_conn_pool: Pool<ConnectionManager<PgConnection>>,
}

impl DbJanitor {
    /// Create a new Database Janitor with a reference to the connection pool.
    pub fn new(conn_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self {
            db_conn_pool: conn_pool,
        }
    }

    /// Call this to clear out duplicate entries from the database.
    /// This is called by the actor once every 24 hours.
    ///
    /// This contains diesel calls that may block, so use carefully.
    fn call(&self) {
        use crate::schema::confirmations::dsl::*;
        use diesel::prelude::*;

        let conn = self
            .db_conn_pool
            .get()
            .map_err(|e| {
                error!("Database Janitor could not get database connection: {}", e);
                e
            })
            .unwrap();

        diesel::delete(confirmations.filter(expiration.le(Local::now())))
            .execute(&conn)
            .map_err(|e| {
                error!(
                    "Could not delete expired confirmations from database: {}",
                    e
                );
                e
            })
            .map(|num| {
                info!("Janitor deleted {} expired email confirmations.", num);
            })
            .unwrap();
    }
}

impl Actor for DbJanitor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Database Janitor Actor started.");
        // call the janitor manually the first time (on start).
        self.call();
        // call the janitor on interval for remainder of uptime.
        let interval = time::Duration::from_secs(60 * 60 * 24); // 24 hours
        ctx.run_interval(interval, |act, _| {
            info!("Database Janitor awoken.");
            act.call();
            info!("Database Janitor sleeping.");
        });
    }
}
