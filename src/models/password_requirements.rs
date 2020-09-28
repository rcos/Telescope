use std::collections::HashSet;

/// Commonly used passwords to protect against.
pub const COMMON_PASSWORDS: &'static [&'static str] = &["password", "12345"];

lazy_static! {
    pub static ref COMMON_PASSWORD_SET: HashSet<&'static str> = {
        let mut hs = HashSet::with_capacity(COMMON_PASSWORDS.len());
        for pass in COMMON_PASSWORDS {
            hs.insert(*pass);
        }
        hs
    };
}

/// Requirements for a valid password.
#[derive(Copy, Clone, Deserialize, Serialize, Debug, juniper::GraphQLObject, Default)]
#[graphql(description = "Requirements for a valid password.")]
pub struct PasswordRequirements {
    /// Password is minimum required length.
    #[graphql(description = "Passwords must satisfy a minimum length or longer.")]
    pub is_min_len: bool,

    /// Password is not one of the commonly used insecure passwords.
    #[graphql(description = "Passwords must not be one of a set of common passwords.")]
    pub not_common_password: bool,
}

impl PasswordRequirements {
    /// Minimum password length.
    pub const MIN_PASSWORD_LENGTH: usize = 5;

    /// Get a list of common passwords.
    pub fn common_passwords() -> &'static [&'static str] {
        &*COMMON_PASSWORDS
    }

    /// Ge the common password set static.
    pub fn common_pasword_set() -> &'static HashSet<&'static str> {
        &*COMMON_PASSWORD_SET
    }

    /// Check if a password satisfies the password requirement set.
    pub fn for_password(pass: &str) -> Self {
        Self {
            is_min_len: pass.len() >= Self::MIN_PASSWORD_LENGTH,
            not_common_password: !COMMON_PASSWORD_SET.contains(pass),
        }
    }

    /// Return true if both password requirements are satisfied.
    pub fn are_satisfied(&self) -> bool {
        self.is_min_len && self.not_common_password
    }
}
