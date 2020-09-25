use super::Email;

#[test]
fn test_email_regex_success() {
    for e in &["test@test.org", "test@test.test.dev"] {
        eprintln!("testing {}", e);
        assert!(Email::get_email_regex().is_match(e));
    }
}

#[test]
fn test_email_regex_failures() {
    for e in &["test"] {
        assert!(!Email::get_email_regex().is_match(e));
    }
}
