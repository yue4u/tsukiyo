mod db;
mod events;
mod gql;
mod redis;
mod schema;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{App, HttpServer};

pub(crate) struct Context {
    pool: db::Pool,
}

impl juniper::Context for Context {}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::create_pool();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(gql::controller::register)
            .service(events::controller::scope())
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}
