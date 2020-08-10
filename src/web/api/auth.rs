/// Commonly used passwords to protect against.
pub const COMMON_PASSWORDS: &'static [&'static str] = &["password", "12345"];

/// Minimum password length is 5 characters.
pub const MIN_LENGTH: usize = 5;

use std::collections::HashSet;
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
    #[graphql(description = "Passwords must satisfy a minimum length or longer.")]
    is_min_len: bool,
    #[graphql(description = "Passwords must not be one of a set of common passwords.")]
    not_common_password: bool,
}

impl PasswordRequirements {
    /// Check if a password satisfies the password requirement set.
    pub fn for_password(pass: &str) -> Self {
        Self {
            is_min_len: pass.len() >= MIN_LENGTH,
            not_common_password: !COMMON_PASSWORD_SET.contains(pass),
        }
    }
}
