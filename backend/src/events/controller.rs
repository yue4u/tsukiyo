use actix_web::{get,post, HttpResponse, Responder};

#[get("/events/list")]
pub async fn list() -> impl Responder {
    HttpResponse::Ok().body(super::service::list())
}

#[post("/events/create")]
pub async fn create() -> impl Responder {
    HttpResponse::Ok().body(super::service::create("",""))
}
