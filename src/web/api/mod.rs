mod root;
pub use root::ApiContext;

mod emails;
pub use emails::Email;

mod users;
pub use users::User;

mod auth;
pub use auth::PasswordRequirements;

mod confirmations;
pub use confirmations::Confirmation;

use actix_web::{web, Error, HttpResponse};

use root::Schema;

use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

use crate::web::RequestContext;

/// Handler for GraphQL API requests.
pub async fn graphql_api(
    ctx: RequestContext,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let api_ctx = ctx.get_api_context();

    let res = web::block(move || {
        let res = data.execute(&api_ctx.schema, &api_ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await
    .map_err(Error::from)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(res))
}

/// Service for interactive GraphQL playground.
pub async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(graphiql_source("/api"))
}

/// Function to register the GraphQl API and playground with actix-web.
pub fn register(config: &mut web::ServiceConfig) {
    config
        .route("/api", web::post().to(graphql_api))
        .route("/playground", web::get().to(graphql_playground));
}
