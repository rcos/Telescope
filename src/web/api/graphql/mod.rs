mod root;
pub use root::ApiContext;

use actix_web::{web, Error, HttpResponse};

use juniper::http::{
    GraphQLRequest,
    graphiql::graphiql_source,
};

use crate::{
    templates::{graphql_playground::GraphQlPlaygroundPage, jumbotron::Jumbotron},
    web::RequestContext,
};

/// Handler for GraphQL API requests.
#[post("/api/graphql")]
pub async fn graphql_api(
    ctx: RequestContext,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    if let Some(api_ctx) = ctx.get_api_context().await {
        // Execute request
        let res = web::block(move || {
            let res = data.execute(&api_ctx.schema, &api_ctx);
            Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
        })
        .await
        .map_err(Error::from)?;

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(res))
    } else {
        Ok(HttpResponse::Unauthorized().body(r#"You must be logged in to make API requests."#))
    }
}

/// Service for interactive GraphQL playground.
///
/// Only available to signed in users. (currently)
#[get("/playground")]
pub async fn graphql_playground(req_ctx: RequestContext) -> HttpResponse {
    if !req_ctx.logged_in().await {
        HttpResponse::Unauthorized()
            .content_type("text/html; charset=utf-8")
            .body(
                Jumbotron::jumbotron_page(
                    &req_ctx,
                    "RCOS - Unauthorized",
                    "Unauthorized",
                    "You must login to access the API playground.",
                )
                .await,
            )
    } else {
        let endpoint = "/api/graphql";

        let playground_page = GraphQlPlaygroundPage::for_endpoint(endpoint);
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(req_ctx.render(&playground_page))
         /*
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(graphiql_source(endpoint))

          */
    }
}

/// Function to register the GraphQl API and playground with actix-web.
pub fn register(config: &mut web::ServiceConfig) {
    config.service(graphql_api).service(graphql_playground);
}
