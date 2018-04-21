#[macro_use]
extern crate diesel;
extern crate diesel_migrations;
extern crate rayon;

mod schema;

use diesel::prelude::*;
use diesel::r2d2;
use rayon::prelude::*;
use std::error::Error;
use std::fs;

const DB_PATH: &str = "test.sqlite";

type ConnectionPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;
type BoxError = Box<Error + Send + Sync>;

#[test]
fn test_connection() {
    establish_connection().unwrap();
}

pub fn establish_connection() -> Result<ConnectionPool, BoxError> {
    fs::remove_file(DB_PATH)?;
    let pool = new_db_pool(DB_PATH)?;
    ::diesel_migrations::run_pending_migrations(&*pool.get()?)?;
    eprintln!("Populating");
    populate(&pool)?;
    Ok(pool)
}

fn new_db_pool(url: &str) -> Result<ConnectionPool, r2d2::PoolError> {
    let manager = r2d2::ConnectionManager::new(url);
    r2d2::Pool::builder()
        // .max_size(1) // uncommenting makes it work
        .build(manager)
}

fn populate(conn: &ConnectionPool) -> Result<(), BoxError> {
    use schema::agg::dsl::*;

    let maybe_err = (0..512)
        .into_par_iter()
        .map(|_| -> Result<_, BoxError> {
            Ok(diesel::replace_into(agg)
                .values((agg_val.eq(0),))
                .execute(&*conn.get()?)?)
        })
        .find_any(Result::is_err);

    if let Some(err) = maybe_err {
        err?;
    }

    Ok(())
}
