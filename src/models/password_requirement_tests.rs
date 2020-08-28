
use super::password_requirements::PasswordRequirements;

#[test]
fn test_common_passwords() {
    for p in &["password"] {
        assert!(!PasswordRequirements::for_password(p).not_common_password);
    }
}

#[test]
fn test_too_short() {
    for p in &["Pass"] {
        assert!(!PasswordRequirements::for_password(p).is_min_len);
    }
}

#[test]
fn test_good_passwords() {
    for p in &["AbcdE123", "7Ns&Z3*g76G@hTRe"] {
        assert!(PasswordRequirements::for_password(p).are_satisfied());
    }
}