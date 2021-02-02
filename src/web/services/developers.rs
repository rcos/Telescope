use actix_web::HttpResponse;
use crate::web::RequestContext;

/// The page displaying RCOS developers.
#[get("/developers")]
pub async fn developers_page(ctx: RequestContext) -> HttpResponse {
    unimplemented!()
}
