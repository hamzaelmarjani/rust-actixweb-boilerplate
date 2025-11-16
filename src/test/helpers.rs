pub fn init_jwt_envs() {
    unsafe {
        std::env::set_var("JWT_SECRET", "u5LdfHsuS1xYxZ8FSg9X5fY3o5w8Rv5MG7ZkUPHTkB4=");
        std::env::set_var(
            "ENCRYPTION_SECRET",
            "Q/ifjccW09g6ZgkQ8HUN1YUVSgGbeDRO6R4bXqjM1V8=",
        );
    }
}

#[allow(dead_code)]
pub fn set_envs(key: &str, value: &str) {
    unsafe {
        std::env::set_var(key, value);
    }
}
