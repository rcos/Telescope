use crate::web::RequestContext;
use actix_web::web::Form;
use actix_web::{web::Path, HttpResponse};
use uuid::Uuid;
use crate::models::Confirmation;
use crate::templates::jumbotron::Jumbotron;

/// The form sent to new users to confirm the account creation.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewUserConfirmation {
    /// The name of the user
    name: String,
    /// The password
    password: String,
    /// The confirmation of the password. This should match the password.
    confirm: String,
}

#[get("/confirm/{invite_id}")]
pub async fn confirmations_page(ctx: RequestContext, Path(invite_id): Path<Uuid>) -> HttpResponse {
    let confirmation = Confirmation::get_by_id(
        ctx.get_db_conn().await,
        invite_id
    ).await.expect("Error getting confirmation.");

    // handle missing or expired confirmation.
    if let Some(confirmation) = confirmation {
        if confirmation.creates_user() {
            unimplemented!()
        } else {
            unimplemented!()
        }
    } else {
        let page = Jumbotron::jumbotron_page(
            &ctx,
            "Not Found",
            "Invite Not Found",
            format!("Could not find confirmation {}. It may have expired.", invite_id)
        ).await;
        HttpResponse::NotFound()
            .body(page)
    }
}

#[post("/confirm/{invite_id}")]
pub async fn confirm(
    ctx: RequestContext,
    Path(invite_id): Path<Uuid>,
    Form(form): Form<NewUserConfirmation>,
) -> HttpResponse {
    let confirmation = Confirmation::get_by_id(
        ctx.get_db_conn().await,
        invite_id
    ).await.expect("Error getting confirmation.");

    // handle missing or expired confirmation.
    if let Some(confirmation) = confirmation {
        if confirmation.creates_user() {
            unimplemented!()
        } else {
            let error_message = format!(
                "Confirmation {} is already linked to an existing user",
                invite_id
            );

            let page = Jumbotron::jumbotron_page(
                &ctx,
                "Cannot create user",
                "Bad request",
                error_message
            ).await;

            return HttpResponse::BadRequest().body(page);
        }
    } else {
        HttpResponse::NotFound()
            .body(Jumbotron::jumbotron_page(
                &ctx,
                "Not Found",
                "Invite Not Found",
                format!("Could not find confirmation {}. It may have expired.", invite_id)
            ).await)
    }
}
