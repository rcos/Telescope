use crate::schema::confirmations;
use chrono::{DateTime, Duration, Local};
use uuid::Uuid;
use crate::web::RequestContext;
use crate::models::Email;

/// An email to a user asking them to confirm their email (and possibly set up an account).
#[derive(Clone, Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "confirmations"]
pub struct Confirmation {
    invite_id: Uuid,
    email: String,
    user_id: Option<Uuid>,
    expiration: DateTime<Local>,
}

impl Confirmation {
    /// Currently invites expire after 30 minutes.
    fn get_expiration_duration() -> Duration {
        Duration::minutes(30)
    }

    /// Get the current datetime and add tge expiration time.
    fn get_expiration_time_from_now() -> DateTime<Local> {
        Local::now() + Self::get_expiration_duration()
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
            expiration: Self::get_expiration_time_from_now()
        }
    }

    /// Create an invite for a new user and store it in the database.
    /// Send an email using the context's mailers to the invited user.
    pub async fn invite_new(ctx: &RequestContext, email: String) -> Result<(), String> {
        let invite = Self::new(email);

        // check that the email is not already registered.
        let conn = ctx.get_db_connection().await;
        let user_exists_for_email =
            Email::get_user_from_db_by_email(conn, invite.email.clone())
                .await
                .is_some();
        if user_exists_for_email {
            return Err(format!("User already exists for email {}", invite.email));
        }

        

        Ok(())
    }
}
