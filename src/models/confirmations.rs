use crate::{
    models::Email,
    schema::confirmations,
    web::{DbConnection, RequestContext},
    templates::emails::confirmation_email::ConfirmationEmail
};
use actix_web::web::block;
use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;
use diesel::result::Error as DieselError;
use actix_web::rt::blocking::BlockingError;

/// An email to a user asking them to confirm their email (and possibly set up an account).
#[derive(Clone, Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "confirmations"]
pub struct Confirmation {
    /// The invitation id
    pub invite_id: Uuid,
    /// The email to confirm.
    email: String,
    /// The user id to associate the email with (if there is one).
    user_id: Option<Uuid>,
    /// When the invite expires.
    pub expiration: DateTime<Utc>,
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
            .map(|_| Err(format!("The email {} is already in use.", invite.email)))
            .unwrap_or(Ok(()))?;

        // check if this user was previously invited.
        Self::check_invite_for(ctx.get_db_conn().await, invite.email.clone())
            .await?
            .filter(|c| !c.is_expired()) // ignore if expired - we will replace.
            .map(|c| {
                Err(format!(
                    "An invite has already been sent to {}. (invite id: {}, exp: {})",
                    c.email, c.invite_id, c.expiration.naive_local()
                ))
            })
            .unwrap_or(Ok(()))?;

        // Get the domain string of the request to use in the email sent to
        // the user.
        let domain = ctx.request()
            .uri()
            .authority()
            .map(|a| format!("https://{}", a.as_str()))
            .ok_or("Could not get domain string.".to_string())
            .map_err(|e| {
                error!("{}", e);
                e
            })?;

        // create invite email. Don't send it yet. Just make sure it can be created
        // without issues.
        let email_builder = lettre_email::Email::builder()
            .from(ctx.email_sender())
            .to(invite.email.as_str())
            .subject("RCOS Email Confirmation");

        let email = ConfirmationEmail::new(domain, invite.invite_id)
            .write_email(ctx, email_builder)?
            .build()
            .map_err(|e| {
                error!("Could not build email: {}", e);
                "Could not create email".to_string()
            })?;

        // if the email is not already invited or in use (or the invite has expired)
        // we can create an invite in the database and send the invite email.
        let conn: DbConnection = ctx.get_db_conn().await;

        // block on database code.
        let saved_confirmation = block::<_, Confirmation, DieselError>(move || {
            use crate::schema::confirmations::dsl::*;
            use diesel::prelude::*;
            diesel::insert_into(confirmations)
                .values(&invite)
                .on_conflict(email)
                .do_update()
                .set((
                    expiration.eq(Utc::now()),
                    user_id.eq(invite.user_id),
                    invite_id.eq(invite.invite_id)
                ))
                .get_result(&conn)
        })
            .await
            // if unsuccessful convert error to string.
            .map_err(move |e| {
                match e {
                    BlockingError::Canceled => error!("Confirmation save canceled"),
                    BlockingError::Error(e) => error!("Could not create confirmation: {}", e),
                }
                "Could not access database.".to_string()
            })?;

        // try to send email.
        let mail_result = ctx.send_mail(email).await;

        // if email failed to send, remove database invite record
        if mail_result.is_err() {
            // more blocking on sync diesel code.
            let conn: DbConnection = ctx.get_db_conn().await;
            let email_ = saved_confirmation.email.clone();
            block::<_, _, DieselError>(move || {
                use crate::schema::confirmations::dsl::*;
                use diesel::prelude::*;
                diesel::delete(confirmations)
                    .filter(email.eq(email_))
                    .execute(&conn)
            })
                .await
                .map_err(|e| {
                    match e {
                        BlockingError::Canceled => error!("Database call canceled"),
                        BlockingError::Error(e) => error!("Could not delete confirmation record: {}", e),
                    }
                    "Could not send email or delete invite. Please notify a sysadmin.".to_string()
                })?;
            return Err(format!("Could not send email to {}", saved_confirmation.email));
        } else {
            Ok(saved_confirmation)
        }
    }
}
