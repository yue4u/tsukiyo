use actix_web::{web, HttpResponse, Responder, Scope};
use super::model::*;
use crate::db::DBPool;

pub fn scope() -> Scope {
    web::scope("/events")
        .route("/list", web::get().to(list))
        .route("/create", web::post().to(create))
}

async fn create(pool:web::Data<DBPool>,event: web::Json<NewEvent>) -> impl Responder {
    println!("post_todo");
    println!("{:?}", event);
    HttpResponse::Ok().body("ok".to_string())
    // HttpResponse::Ok().body(super::service::create(pool.get().expect("couldn't get db connection from pool"),"event", "this is a test event"))
}

async fn list(pool:web::Data<DBPool>) -> impl Responder {
    HttpResponse::Ok().body(super::service::list(pool.get().expect("couldn't get db connection from pool")))
}
