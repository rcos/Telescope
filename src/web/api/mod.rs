mod root;
pub use root::ApiContext;

pub use crate::models::password_requirements::PasswordRequirements;

use actix_web::{web, Error, HttpResponse};


use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

use crate::web::RequestContext;
use crate::templates::jumbotron::Jumbotron;

/// Handler for GraphQL API requests.
pub async fn graphql_api(
    ctx: RequestContext,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    if let Some(api_ctx) = ctx.get_api_context() {
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
        Ok(HttpResponse::Unauthorized()
            .body("You must be logged in to make API requests."))
    }
}

/// Service for interactive GraphQL playground.
///
/// Only available to signed in users.
pub async fn graphql_playground(req_ctx: RequestContext) -> HttpResponse {
    if !req_ctx.logged_in() {
        HttpResponse::Unauthorized()
            .content_type("text/html; charset=utf-8")
            .body(Jumbotron::jumbotron_page(
                &req_ctx,
                "RCOS - Unauthorized",
                "Unauthorized",
                "You must login to access the API playground."
            ))
    } else {
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(graphiql_source("/api"))
    }
}

/// Function to register the GraphQl API and playground with actix-web.
pub fn register(config: &mut web::ServiceConfig) {
    config
        .route("/api", web::post().to(graphql_api))
        .route("/playground", web::get().to(graphql_playground));
}
