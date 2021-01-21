mod auth;
mod contacts;
mod events;
mod gql;
mod sql;
mod utils;

extern crate openssl;
#[macro_use]
extern crate diesel;
use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpRequest, HttpServer, Responder};
pub struct Context {
    pool: sql::db::Pool,
    user: Option<auth::User>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    openssl_probe::init_ssl_cert_env_vars();
    let pool = sql::db::create_pool();

    HttpServer::new(move || {
        #[cfg(debug_assertions)]
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:4000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);

        #[cfg(not(debug_assertions))]
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_origin("http://localhost:3000")
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .data(pool.clone())
            .route("/ping", web::get().to(pong))
            .configure(gql::controller::register)
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}

async fn pong() -> impl Responder {
    format!("/pong")
}
