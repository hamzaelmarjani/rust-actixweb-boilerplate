pub fn get_env(key: &str) -> String {
    match std::env::var(key) {
        Ok(val) => val,
        Err(_) => {
            std::process::exit(1);
        }
    }
}
