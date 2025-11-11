#[cfg(test)]
mod tests {
    use crate::structs::jwt::JwtManager;
    #[test]
    fn test_encode_jwt() {
        std::env::set_var("JWT_SECRET", "u5LdfHsuS1xYxZ8FSg9X5fY3o5w8Rv5MG7ZkUPHTkB4=");
        std::env::set_var("ENCRYPTION_SECRET", "Q/ifjccW09g6ZgkQ8HUN1YUVSgGbeDRO6R4bXqjM1V8=");

        let encoded = JwtManager::new()
            .unwrap()
            .encode_jwt(&"SOME_DATA_TO_ENCODE", None);

        assert!(encoded.is_ok());
    }

    #[test]
    fn test_decode_jwt() {
        std::env::set_var("JWT_SECRET", "u5LdfHsuS1xYxZ8FSg9X5fY3o5w8Rv5MG7ZkUPHTkB4=");
        std::env::set_var("ENCRYPTION_SECRET", "Q/ifjccW09g6ZgkQ8HUN1YUVSgGbeDRO6R4bXqjM1V8=");

        let decoded = JwtManager::new()
            .unwrap()
            .decode_jwt_and_decrypt(&"INVALIDE_TOKEN");

        assert!(!decoded.is_ok());
    }
}
