
use actix_web::{
    web::Json,
    Result as ActixResult
};

use actix_identity::Identity;

use crate::{
    models::Email,
    web::RequestContext
};

/// Login request object
#[derive(Clone, Debug, Serialize, Deserialize)]
struct LoginRequest {
    email: String,
    password: String
}

/// Login response object
#[derive(Clone, Serialize, Debug, Deserialize)]
enum LoginResponse {
    Ok,
    EmailNotFound,
    WrongPassword
}

#[post("/api/rest/login")]
pub async fn login(ctx: RequestContext, data: Json<LoginRequest>) -> ActixResult<Json<LoginResponse>> {
    let identity: &Identity = ctx.identity();
    let LoginRequest { email, password} = data.into_inner();
    let target_user = Email::get_user_from_db_by_email(
        ctx.get_db_connection().await,
        email
    ).await;

    if let Some(target) = target_user {
        unimplemented!()
    } else {
        Ok(Json(LoginResponse::EmailNotFound))
    }
}