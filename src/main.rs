// main.rs
// to avoid the warning from diesel macros
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
extern crate bcrypt;
extern crate jsonwebtoken as jwt;
#[macro_use]
extern crate juniper;

mod app;
mod auth_handler;
mod auth_routes;
mod errors;
mod graph;
mod invitation_handler;
mod invitation_routes;
mod models;
mod register_handler;
mod register_routes;
mod schema;
mod utils;

#[allow(unused_imports)]
use crate::graph::routes::graphiql;
use crate::models::DbExecutor;
use ::actix::prelude::*;
use ::actix_web::server;
use ::diesel::{r2d2::ConnectionManager, PgConnection};
use ::dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "simple-auth-server=debug,actix_web=info");
    env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let sys = actix::System::new("Actix_Tutorial");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let db_address: Addr<DbExecutor> = SyncArbiter::start(4, move || DbExecutor(pool.clone()));

    let schema = std::sync::Arc::new(crate::graph::schema::create_schema());
    let graph_addrss = SyncArbiter::start(3, move || {
        crate::graph::routes::GraphQLExecutor::new(schema.clone())
    });

    server::new(move || app::create_app(db_address.clone(), graph_addrss.clone()))
        .bind("127.0.0.1:3000")
        .expect("Can not bind to '127.0.0.1:3000'")
        .start();

    sys.run();
}
