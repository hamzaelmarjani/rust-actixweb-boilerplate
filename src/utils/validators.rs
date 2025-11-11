use regex::Regex;

pub fn is_valid_email(email: &str) -> bool {
    let pattern = r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(email)
}