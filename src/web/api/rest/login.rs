use actix_web::web::Json;

use actix_identity::Identity;

use crate::{
    models::{Email, User},
    web::RequestContext,
};

/// Login request object
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    password: String,
}

/// Login response object
#[derive(Clone, Serialize, Debug, Deserialize)]
pub enum LoginError {
    EmailNotFound,
    WrongPassword,
}

/// Login logic. Uses ctx only for database connections.
/// Does not modify identity.
pub async fn login(ctx: &RequestContext, request: LoginRequest) -> Result<User, LoginError> {
    let LoginRequest { email, password } = request;
    let target_user = Email::get_user_from_db_by_email(ctx.get_db_connection().await, email).await;

    if let Some(target) = target_user {
        let pass = password.as_bytes();
        let verified = argon2::verify_encoded(target.hashed_pwd.as_str(), pass)
            .map_err(|e| {
                error!("Argon2 Verification Error: {}", e);
            })
            .unwrap_or(false);
        if verified {
            Ok(target)
        } else {
            Err(LoginError::WrongPassword)
        }
    } else {
        Err(LoginError::EmailNotFound)
    }
}

/// The Login REST API.
/// Modifies identity appropriately.
#[post("/api/rest/login")]
pub async fn login_rest(
    ctx: RequestContext,
    data: Json<LoginRequest>,
) -> Json<Result<User, LoginError>> {
    let identity: &Identity = ctx.identity();
    let result = login(&ctx, data.into_inner()).await.map(|t| {
        identity.remember(t.id_str());
        t
    });
    Json(result)
}
