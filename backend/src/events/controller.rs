use super::model::*;
use crate::db::DBPool;
use actix_web::{web, Responder, Scope};

pub fn scope() -> Scope {
    web::scope("/events")
        .route("/create", web::post().to(create))
        .route("/delete", web::post().to(delete))
        .route("/update", web::post().to(update))
        .route("/list", web::get().to(list))
}

async fn create(pool: web::Data<DBPool>, event: web::Json<EventInput>) -> impl Responder {
    // TODO: use web::block
    // https://github.com/actix/examples/blob/91865b3a2ad175166f472f8b3393eae1a1eb5996/diesel/src/main.rs#L55
    super::service::create(
        pool.get().expect("couldn't get db connection from pool"),
        event.into_inner(),
    )
}

async fn update(pool: web::Data<DBPool>, event: web::Json<EventUpdate>) -> impl Responder {
    super::service::update(
        pool.get().expect("couldn't get db connection from pool"),
        event.into_inner(),
    )
}

async fn delete(pool: web::Data<DBPool>, event_id: web::Json<i32>) -> impl Responder {
    super::service::delete(
        pool.get().expect("couldn't get db connection from pool"),
        event_id.into_inner(),
    )
}

async fn list(pool: web::Data<DBPool>) -> impl Responder {
    super::service::list(pool.get().expect("couldn't get db connection from pool"))
}
