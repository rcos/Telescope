
use actix_web::web::Json;

use actix_identity::Identity;

use crate::{
    models::Email,
    web::RequestContext
};

/// Login request object
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String
}

/// Login response object
#[derive(Clone, Serialize, Debug, Deserialize)]
pub enum LoginResponse {
    Ok,
    EmailNotFound,
    WrongPassword
}

#[post("/api/rest/login")]
pub async fn login(ctx: RequestContext, data: Json<LoginRequest>) -> Json<LoginResponse> {
    let identity: &Identity = ctx.identity();
    let LoginRequest { email, password} = data.into_inner();
    let target_user = Email::get_user_from_db_by_email(
        ctx.get_db_connection().await,
        email
    ).await;

    if let Some(target) = target_user {
        let pass = password.as_bytes();
        let verified = argon2::verify_encoded(target.hashed_pwd.as_str(), pass)
            .map_err(|e| {
                error!("Argon2 Verification Error: {}", e);
            })
            .unwrap_or(false);
        if verified {
            identity.remember(target.id_str());
            Json(LoginResponse::Ok)
        } else {
            Json(LoginResponse::WrongPassword)
        }
    } else {
        Json(LoginResponse::EmailNotFound)
    }
}