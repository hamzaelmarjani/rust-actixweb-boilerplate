use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref PUBLIC_ROUTES: Vec<Regex> = vec![
        Regex::new(r"^/api/user/sign-in$").unwrap(),
        Regex::new(r"^/api/user/sign-up$").unwrap(),
        Regex::new(r"^/api/verify/email$").unwrap(),
    ];
}
