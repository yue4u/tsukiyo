use super::model::*;
use crate::db::Pool;
use actix_web::{web, Responder, Scope};

pub fn scope() -> Scope {
    web::scope("/events")
        .route("/create", web::post().to(create))
        .route("/delete", web::post().to(delete))
        .route("/update", web::post().to(update))
        .route("/list", web::get().to(list))
}

async fn create(pool: web::Data<Pool>, event: web::Json<EventInput>) -> impl Responder {
    // TODO: use web::block
    // https://github.com/actix/examples/blob/91865b3a2ad175166f472f8b3393eae1a1eb5996/diesel/src/main.rs#L55
    let event = super::service::create(
        pool.get().expect("couldn't get db connection from pool"),
        event.into_inner(),
    )
    .expect("failed to create event");
    serde_json::to_string(&event)
}

async fn update(pool: web::Data<Pool>, event: web::Json<EventUpdate>) -> impl Responder {
    let updates = super::service::update(
        pool.get().expect("couldn't get db connection from pool"),
        // TODO: fix this
        0,
        event.into_inner(),
    )
    .expect("failed to update event");
    serde_json::to_string(&updates)
}

async fn delete(pool: web::Data<Pool>, event_id: web::Json<i32>) -> impl Responder {
    let event = super::service::delete(
        pool.get().expect("couldn't get db connection from pool"),
        event_id.into_inner(),
    )
    .expect("failed to delete event");
    serde_json::to_string(&event)
}

async fn list(pool: web::Data<Pool>) -> impl Responder {
    let events = super::service::list(pool.get().expect("couldn't get db connection from pool"))
        .expect("failed to list event");
    serde_json::to_string(&events)
}
