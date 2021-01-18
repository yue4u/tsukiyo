use actix_web::{web, HttpRequest, HttpResponse, Responder};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

use super::schema::{create_schema, Schema};
use crate::Context;
use crate::{auth, sql::db::Pool};

pub(crate) async fn graphql(
    req: HttpRequest,
    pool: web::Data<Pool>,
    schema: web::Data<Schema>,
    data: web::Json<GraphQLRequest>,
) -> impl Responder {
    let ctx = Context {
        pool: pool.get_ref().to_owned(),
        user: auth::get_user(&req).await.ok(),
    };
    let res = data.execute(&schema, &ctx).await;
    serde_json::to_string(&res).expect("failed to get data")
}

pub async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(graphiql_source("/graphql", None))
}

pub fn register(config: &mut web::ServiceConfig) {
    config
        .data(create_schema())
        .route("/graphql", web::post().to(graphql))
        .route("/graphiql", web::get().to(graphiql));
}
