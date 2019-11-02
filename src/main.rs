#[macro_use]
extern crate nickel;
extern crate nickel_postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate bcrypt;
extern crate rand;

use nickel::{Nickel, Mountable, StaticFilesHandler};
use nickel_postgres::PostgresMiddleware;
use r2d2::Pool;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};

use std::env;

pub mod resource;
use resource::account::controller as account_controller;

pub mod engine;
use engine::log_engine;

fn main() {
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    let addr = env::var("ADDR").unwrap();
    let port = env::var("PORT").unwrap();
    let db_uri = env::var("DB_URI").unwrap();

    // Serve front end
    // TODO: Disable this for CDN settings
    server.mount("/app/", StaticFilesHandler::new("front/app/")
    );

    // Initialize database
    let db_mgr = PostgresConnectionManager::new(db_uri.as_ref(), TlsMode::None) // TODO: Investigate TlsMode
        .expect("Unable to connect to database");
    let db_pool = Pool::new(db_mgr)
        .expect("Unable to initialize connection pool");
    server.utilize(PostgresMiddleware::with_pool(db_pool));

    // Init Logger
    log_engine::attache(&mut server);

    // Init controllers
    account_controller::add_route(&mut router);
    server.utilize(router);

    server.listen(format!("{}:{}", addr, port)).unwrap();
}