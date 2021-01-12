mod root;
pub use root::ApiContext;

use actix_web::{web, Error, HttpResponse};

use juniper::http::GraphQLRequest;

use crate::{
    templates::{graphql_playground, jumbotron, Template},
    web::RequestContext,
};

/// Handler for GraphQL API requests.
#[post("/api/graphql")]
pub async fn api(
    ctx: RequestContext,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    if let Some(api_ctx) = ctx.get_api_context().await {
        // Execute request
        let res = data.execute(&api_ctx.schema, &api_ctx).await;
        let json_str = serde_json::to_string(&res).map_err(Error::from)?;

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(json_str))
    } else {
        Ok(HttpResponse::Unauthorized().body(r#"You must be logged in to make API requests."#))
    }
}

/// Service for interactive GraphQL playground.
///
/// Only available to signed in users. (currently)
#[get("/playground")]
pub async fn playground(req_ctx: RequestContext) -> HttpResponse {
    if !req_ctx.logged_in().await {
        let jumbo: Template = jumbotron::new(
            "Unauthorized",
            "You must login to access the API playground.",
        );

        HttpResponse::Unauthorized()
            .content_type("text/html; charset=utf-8")
            .body(req_ctx.render_in_page(&jumbo, "RCOS - Unauthorized").await)
    } else {
        let endpoint = "/api/graphql";

        let playground_page: Template = graphql_playground::for_endpoint(endpoint);
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(req_ctx.render(&playground_page))
    }
}

/// Function to register the GraphQl API and playground with actix-web.
pub fn register(config: &mut web::ServiceConfig) {
    config.service(api).service(playground);
}
