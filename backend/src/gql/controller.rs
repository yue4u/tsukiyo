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
    #[cfg(debug_assertions)]
    let res = data.execute(&schema.admin, &ctx).await;

    #[cfg(not(debug_assertions))]
    let res = if ctx.user.is_some() {
        data.execute(&schema.admin, &ctx).await
    } else {
        data.execute(&schema.public, &ctx).await
    };

    serde_json::to_string(&res).expect("failed to get data")
}

pub async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(graphiql_source("/graphql", None))
}

pub fn register(config: &mut web::ServiceConfig) {
    let config = config
        .data(create_schema())
        .route("/graphql", web::post().to(graphql));
    #[cfg(debug_assertions)]
    config.route("/graphiql", web::get().to(graphiql));
}
