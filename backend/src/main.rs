mod events;
mod gql;
mod sql;
#[macro_use]
extern crate diesel;
use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, App, HttpServer};

pub(crate) struct Context {
    pool: sql::db::Pool,
}

impl juniper::Context for Context {}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = sql::db::create_pool();
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_origin("http://localhost:4000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .data(pool.clone())
            .configure(gql::controller::register)
            .service(events::controller::scope())
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}
