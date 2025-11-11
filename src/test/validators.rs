#[cfg(test)]
mod tests {
    use crate::utils::env::get_env;
    use crate::utils::validators::is_valid_email;
    #[test]
    fn test_valid_emails() {
        assert!(is_valid_email("hamzaelmarjani@gmail.com"));
    }

    #[test]
    fn test_invalid_emails() {
        assert!(!is_valid_email("plainaddress"));
    }

    #[test]
    fn test_accessable_env_var() {
        std::env::set_var("UNIT_TEST", "ACCESSABLE");
        assert_eq!(get_env("UNIT_TEST"), "ACCESSABLE");
    }

    #[test]
    fn test_unaccessable_env_var() {
        std::env::set_var("UNIT_TEST", "SOMETHING_ELSE");
        assert_ne!(get_env("UNIT_TEST"), "ACCESSABLE");
    }
}
