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
extern crate chrono;
extern crate plugin;
extern crate rand;
extern crate typemap;

use nickel::{Mountable, Nickel, StaticFilesHandler, Router};
use nickel_postgres::PostgresMiddleware;
use r2d2::Pool;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};

use std::env;

pub mod resource;
use resource::account::controller as account_controller;

pub mod engine;
use engine::log_engine;
use engine::response_engine;
use engine::session_engine;
use engine::config_engine::Config;

fn main() {
    let conf = Config::new();
    let mut server = Nickel::with_data(conf);
    let mut router: Router<Config> = Nickel::router();

    let addr = env::var("ADDR").unwrap();
    let port = env::var("PORT").unwrap();
    let db_uri = env::var("DB_URI").unwrap();
    let session_secret = env::var("SESSION_SECRET").unwrap();

    // Serve front end
    // TODO: Disable this for CDN settings?
    server.mount("/app/", StaticFilesHandler::new("front/app/"));

    // Add the session check to the middleware stack
    session_engine::attach(&mut server, &session_secret);

    // Attach the initialization of the response engine
    // TODO: Find a way to groupe the two response engine call in the main (one MW as to run first, the other one last)
    response_engine::attache_init(&mut server);

    // Initialize database
    let db_mgr = PostgresConnectionManager::new(db_uri.as_ref(), TlsMode::None) // TODO: Investigate TlsMode
        .expect("Unable to connect to database");
    let db_pool = Pool::new(db_mgr).expect("Unable to initialize connection pool");
    server.utilize(PostgresMiddleware::with_pool(db_pool));

    // Init Logger
    log_engine::attache(&mut server);

    // Init controllers
    session_engine::register_session_route(&mut router, &session_secret);
    account_controller::add_route(&mut router);
    server.utilize(router);

    response_engine::attache_final(&mut server);
    server.listen(format!("{}:{}", addr, port)).unwrap();
}
