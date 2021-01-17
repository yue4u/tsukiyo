mod db;
mod events;
mod gql;
mod redis;
mod schema;
mod schema_gql;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/view")]
async fn view() -> impl Responder {
    redis::incr_page_views().unwrap();
    HttpResponse::Ok().body(redis::get_page_views().unwrap().to_string())
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

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
            .configure(gql::register)
            .service(hello)
            .service(echo)
            .service(events::controller::scope())
            .service(view)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}
