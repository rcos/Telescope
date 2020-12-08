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
pub struct PasswordRequirements {
    /// Password is minimum required length.
    pub is_min_len: bool,

    /// Password is under the maximum length.
    pub under_max_length: bool,

    /// Password is not one of the commonly used insecure passwords.
    pub not_common_password: bool,
}

impl PasswordRequirements {
    /// Minimum password length.
    pub const MIN_PASSWORD_LENGTH: usize = 5;

    /// Maximum password length.
    pub const MAX_PASSWORD_LENGTH: usize = 10_000;

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
            under_max_length: pass.len() < Self::MAX_PASSWORD_LENGTH,
            not_common_password: !COMMON_PASSWORD_SET.contains(pass),
        }
    }

    /// Return true if all password requirements are satisfied.
    pub fn satisfied(&self) -> bool {
        self.is_min_len && self.not_common_password && self.under_max_length
    }

    /// Return a string describing any issues with this password.
    pub fn get_error_string(&self) -> Option<String> {
        if !self.is_min_len {
            Some(format!("Password is too short. Minimum length: {}", Self::MIN_PASSWORD_LENGTH))
        } else if !self.under_max_length {
            Some(format!("Password is too long. Maximum length: {}", Self::MAX_PASSWORD_LENGTH))
        } else if !self.not_common_password {
            Some("Password too common. Please pick a more complicated password.".to_string())
        } else {
            None
        }
    }
}
