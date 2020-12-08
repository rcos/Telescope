use crate::{
    models::{emails::Email, password_requirements::PasswordRequirements, users::User},
    schema::confirmations,
    templates::emails::confirmation_email::ConfirmationEmail,
    util::handle_blocking_err,
    web::{DbConnection, RequestContext},
};
use actix_web::web::block;
use chrono::{DateTime, Duration, Utc};
use diesel::result::Error as DieselError;
use uuid::Uuid;

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

/// An error that can be caught in trying to confirm a new user.
///
/// There shouldn't be any email related errors as the confirmation needs to
/// be for a new email to exist.
#[derive(Clone, Debug)]
pub enum ConfirmNewUserError {
    /// User had bad password.
    BadPassword(PasswordRequirements),
    /// Other error occurred.
    Other(String),
}

impl From<String> for ConfirmNewUserError {
    fn from(s: String) -> Self {
        Self::Other(s)
    }
}

impl From<PasswordRequirements> for ConfirmNewUserError {
    fn from(e: PasswordRequirements) -> Self {
        Self::BadPassword(e)
    }
}

impl Confirmation {
    /// Currently invites expire after 10 minutes.
    fn get_expiration_duration() -> Duration {
        Duration::minutes(5)
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
            .map(|c| Err(format!("An invite has already been sent to {}.", c.email)))
            .unwrap_or(Ok(()))?;

        // Get the domain string of the request to use in the email sent to
        // the user.
        let domain = ctx
            .request()
            .uri()
            .authority()
            .map(|a| format!("https://{}", a.as_str()))
            // since the lettre crate file transport serializes as JSON
            // and not .eml, it is hard to determine the generated address.
            // Until this changes, we log it in a trace message so that it's
            // easier to test on development without a full SMTP backend.
            // See https://github.com/lettre/lettre/pull/505.
            .map(|url: String| {
                trace!("Generated Invite URL: {}/confirm/{}", url, invite.invite_id);
                url
            })
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

        let email = ConfirmationEmail::new(domain, &invite)
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
                    invite_id.eq(invite.invite_id),
                ))
                .get_result(&conn)
        })
        .await
        // if unsuccessful convert error to string.
        .map_err(|e| handle_blocking_err(e, "Could not access confirmations database."))?;

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
                handle_blocking_err(
                    e,
                    "Could not send email or delete invite. Please notify a sysadmin.",
                )
            })?;
            return Err(format!(
                "Could not send email to {}",
                saved_confirmation.email
            ));
        } else {
            Ok(saved_confirmation)
        }
    }

    /// Get a confirmation by id. If there is an error, it will be logged and a string describing
    /// it will be returned.
    pub async fn get_by_id(db_conn: DbConnection, inv_id: Uuid) -> Result<Option<Self>, String> {
        block::<_, Option<Confirmation>, _>(move || {
            use crate::schema::confirmations::dsl::*;
            use diesel::prelude::*;
            confirmations
                .filter(invite_id.eq(inv_id))
                .first::<Confirmation>(&db_conn)
                .optional()
        })
        .await
        .map_err(|e| handle_blocking_err(e, "Could not query database."))
        // filter out expired invite
        .map(|opt| opt.filter(|c| !c.is_expired()))
    }

    /// Private function to remove a confirmation from the database.
    async fn remove_from_db(&self, conn: DbConnection) -> Result<(), String> {
        trace!("Removing {:?} from database.", &self);
        let self_id: Uuid = self.invite_id;
        block::<_, usize, _>(move || {
            use crate::schema::confirmations::dsl::*;
            use diesel::prelude::*;
            diesel::delete(confirmations)
                .filter(invite_id.eq(self_id))
                .execute(&conn)
        })
        .await
        .map_err(|e| handle_blocking_err(e, "Could not delete confirmation"))
        .map(|removed| {
            trace!("Removed {} record from confirmations database.", removed);
        })
    }

    /// Try to confirm a new user invite.
    /// Assume that `self` is a valid invite in the confirmations table.
    /// Return the created user or a string describing the error.
    pub async fn confirm_new(
        &self,
        ctx: &RequestContext,
        name: String,
        pass: String,
    ) -> Result<User, ConfirmNewUserError> {
        if !self.creates_user() {
            return Err("Invite is for existing user".to_string().into());
        } else {
            // create user and email here to avoid use after move.
            let user: User = User::new(name, pass.as_str())
                .map_err::<ConfirmNewUserError, _>(|e| e.into())?;

            let email: Email = Email::new(user.id, self.email.clone())
                .expect("Could not create email for new user.");

            // remove confirmation from database.
            self.remove_from_db(ctx.get_db_conn().await).await?;

            // save email and user records.
            user.clone().store(ctx.get_db_conn().await).await?;
            email.store(ctx.get_db_conn().await).await?;
            Ok(user)
        }
    }

    /// Confirm an invite for an existing user.
    /// Return a string describing the error if one occurs.
    pub async fn confirm_existing(&self, ctx: &RequestContext) -> Result<(), String> {
        if self.creates_user() {
            return Err("Invite is not associated with existing user".to_string());
        } else {
            // Construct the email instance here to avoid use after move.
            let email = Email::new(self.user_id.unwrap(), self.email.clone())
                .expect("Could not make email instance.");

            // remove confirmation from database
            self.remove_from_db(ctx.get_db_conn().await).await?;

            // add email to emails table
            email.store(ctx.get_db_conn().await).await
        }
    }
}
