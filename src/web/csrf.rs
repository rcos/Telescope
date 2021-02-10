//! Cross Site Request Forging protection via a global static DashMap.

use dashmap::DashMap;
use std::sync::Arc;
use oauth2::CsrfToken;
use actix_web::HttpRequest;
use crate::error::TelescopeError;
use chrono::{Duration, Utc, DateTime};
use actix::{Actor, Context, AsyncContext};
use std::time::Duration as StdDuration;

lazy_static! {
    static ref GLOBAL_CSRF_MAP: Arc<DashMap<String, (CsrfToken, DateTime<Utc>)>> = Arc::new(DashMap::new());
}

/// Get the global lazy static CSRF map.
fn global_csrf_map() -> Arc<DashMap<String, (CsrfToken, DateTime<Utc>)>> {
    GLOBAL_CSRF_MAP.clone()
}

/// Save a CSRF token linked to the remote IP of the Http Request that created it.
pub fn save_csrf(req: &HttpRequest, token: CsrfToken) -> Result<(), TelescopeError> {
    // Get the remote IP address string.
    let ip_addr: String = req.connection_info()
        .remote_addr()
        .ok_or(TelescopeError::CSRFSaveError)?
        .into();

    // Get the current time and add the expiration duration (10 minutes) to get the
    // expiration time.
    let expiration_time: DateTime<Utc> = Utc::now() + Duration::minutes(10);
    unimplemented!()
}


/// A zero sized struct to act as an actor and run every hour cleaning up
/// expired CSRF tokens.
pub struct CsrfJanitor;

impl CsrfJanitor {
    // Run once every 20 minutes. Return the number of expired
    // CSRF tokens removed from the global hashmap.
    fn call(&self) -> usize {
        // Get a list of keys to remove.
        let remove_keys: Vec<String> = global_csrf_map()
            .iter()
            // Filter for expired records
            .filter(|record| record.value().1 < Utc::now())
            .map(|record| record.key().clone())
            .collect();

        // Remove all the records necessary from the global CSRF map.
        // Return the number of keys removed.
        return remove_keys.iter()
            .map(|ip_addr| global_csrf_map().remove(ip_addr))
            .filter(Option::is_some)
            .count();
    }
}

impl Actor for CsrfJanitor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("CSRF Janitor Started");

        // Set the janitor to get called every 20 minutes.
        let interval: StdDuration = StdDuration::new(20 * 60, 0);

        ctx.run_interval(interval, |actor, _| {
            info!("Calling CSRF Janitor.");
            let removed: usize = actor.call();
            info!("CSRF Janitor removed {} tokens.", removed);
        });
    }
}