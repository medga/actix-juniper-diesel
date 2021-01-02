use actix_web::{web, Error, HttpResponse};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use crate::graphql::root::{create_schema, Context, Schema};

pub async fn graphql(
    schema: web::Data<Schema>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = Context {};
    let req = GraphQLRequest::from(data.into_inner());
    let gql_response = req.execute(&schema, &ctx).await;
    let body_response = serde_json::to_string(&gql_response)?;
    let mut response = match gql_response.is_ok() {
        true => HttpResponse::Ok(),
        false => HttpResponse::BadRequest(),
    };

    Ok(response
        .content_type("application/json")
        .body(body_response))
}

pub async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(graphiql_source("/", None))
}

pub fn register(config: &mut web::ServiceConfig) {
    config
        .data(create_schema())
        .route("/", web::post().to(graphql))
        .route("/iql", web::get().to(graphql_playground));
}
