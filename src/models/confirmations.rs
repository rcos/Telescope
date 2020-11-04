use crate::{
    models::Email,
    schema::confirmations,
    web::{DbConnection, RequestContext},
};
use actix_web::web::block;
use chrono::{DateTime, Duration, Utc};
use diesel::RunQueryDsl;
use futures::prelude::*;
use uuid::Uuid;

/// An email to a user asking them to confirm their email (and possibly set up an account).
#[derive(Clone, Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "confirmations"]
pub struct Confirmation {
    invite_id: Uuid,
    email: String,
    user_id: Option<Uuid>,
    expiration: DateTime<Utc>,
}

impl Confirmation {
    /// Currently invites expire after 30 minutes.
    fn get_expiration_duration() -> Duration {
        Duration::minutes(30)
    }

    /// Get the current datetime and add tge expiration time.
    fn get_expiration_time_from_now() -> DateTime<Utc> {
        Utc::now() + Self::get_expiration_duration()
    }

    /// Does this invite create a new user, requiring the creation of a password
    pub fn creates_user(&self) -> bool {
        self.user_id.is_none()
    }

    /// Create a new email confirmation/invite that will create a new user.
    fn new(email: String) -> Self {
        let invite_id = Uuid::new_v4();
        Self {
            invite_id,
            email,
            user_id: None,
            expiration: Self::get_expiration_time_from_now(),
        }
    }

    /// Check if this confirmation is expired.
    fn is_expired(&self) -> bool {
        self.expiration <= Utc::now()
    }

    /// Check if an email has been invited already.
    /// This does not check if an email has been registered.
    /// The found invite may be expired.
    async fn check_invite_for(
        conn: DbConnection,
        email_: String,
    ) -> Result<Option<Confirmation>, String> {
        block(move || {
            use crate::schema::confirmations::dsl::*;
            use diesel::prelude::*;
            confirmations
                .find(email_)
                .first::<Confirmation>(&conn)
                .optional()
        })
        .await
        .map_err(|err| {
            error!("Error checking for invite: {}", err);
            "Could not access invite database".to_string()
        })
    }

    /// Create an invite for a new user and store it in the database.
    /// Send an email using the context's mailers to the invited user.
    ///
    /// On success, returns the invite. Otherwise returns a string
    /// summarizing the error encountered.
    pub async fn invite_new(ctx: &RequestContext, email: String) -> Result<Confirmation, String> {
        let invite = Self::new(email);

        // check that the email is not already registered.
        Email::get_user_from_db_by_email(ctx.get_db_conn().await, invite.email.clone())
            .await
            .map(|u| Err(format!("The email {} is already in use.", invite.email)))
            .unwrap_or(Ok(()))?;

        // check if this user was previously invited.
        Self::check_invite_for(ctx.get_db_conn().await, invite.email.clone())
            .await?
            .filter(|c| !c.is_expired()) // ignore if expired - we will replace.
            .map(|c| {
                Err(format!(
                    "An invite has already been sent to {}. (invite id: {})",
                    c.email, c.invite_id
                ))
            })
            .unwrap_or(Ok(()))?;

        // if the email is not already invited or in use (or the invite has expired)
        // we can create an invite in the database and send the invite email.
        let conn = ctx.get_db_conn().await;
        let stored_invite: Confirmation = block::<_, Confirmation, _>(move || {
            use crate::schema::confirmations::dsl::*;
            use diesel::prelude::*;

            diesel::insert_into(confirmations)
                .values(&invite)
                .on_conflict(email)
                .do_update()
                .set((expiration.eq(Utc::now()), user_id.eq(invite.user_id)))
                .get_result(&conn)
        })
        .map_err(|e| {
            error!("Error creating invite in database: {}", e);
            "Could not access invite database.".to_string()
        })
        .await?;

        // create invite email.
        let email = lettre_email::Email::builder()
            .from(ctx.email_sender())
            .to(stored_invite.email.as_str())
            .subject("RCOS Telescope Email Confirmation")
            // FIXME: Write actual emails here.
            // FIXME: Also we need a place to verify the responses. (/confirm/{invite_id})
            .alternative("Html invite", "Plaintext Invite")
            .build()
            .map_err(|e| {
                error!("Could not build email: {}", e);
                "Could not create email".to_string()
            })?;

        ctx.send_mail(email)
            .await
            .map_err(|_| "Could not send email.".to_string())?;

        Ok(stored_invite)
    }
}
