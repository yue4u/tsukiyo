use actix_web::{web, HttpResponse, Responder};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use crate::db::Pool;
use crate::schema_gql::{create_schema, Schema};
use crate::Context;

pub(crate) async fn graphql(
    pool: web::Data<Pool>,
    schema: web::Data<Schema>,
    data: web::Json<GraphQLRequest>,
) -> impl Responder {
    let ctx = Context {
        pool: pool.get_ref().to_owned(),
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
