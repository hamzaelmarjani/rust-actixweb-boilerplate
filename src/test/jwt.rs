#[cfg(test)]
mod tests {
    use crate::structs::jwt::JwtManager;
    use crate::test::helpers::init_jwt_envs;

    #[test]
    fn test_encode_jwt() {
        init_jwt_envs();

        let encoded = JwtManager::new()
            .unwrap()
            .encode_jwt(&"SOME_DATA_TO_ENCODE", None);

        assert!(encoded.is_ok());
    }

    #[test]
    fn test_decode_jwt() {
        init_jwt_envs();

        let decoded = JwtManager::new()
            .unwrap()
            .decode_jwt_and_decrypt(&"INVALIDE_TOKEN");

        assert!(!decoded.is_ok());
    }
}
