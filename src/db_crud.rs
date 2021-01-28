//! CRUD (Create, Read, Update, Delete) methods/traits for items in the
//! Database.

use diesel::pg::Pg;
use diesel::Queryable;

/// Telescope uses a Postgres Database.
type Db = Pg;

/// The base trait for models that support database CRUD operations.
pub trait DbBase {
    /// The error string that gets reported when something goes wrong accessing the database.
    const ERROR: &'static str;
}

pub trait DbCreate {

}

pub trait DbRead {
    
}

pub trait DbUpdate {

}

pub trait DbDelete {

}