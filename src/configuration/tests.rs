#[cfg(test)]
mod tests {
    use crate::configuration::config::{load, Config};
    use std::error::Error;
    use std::fs::{remove_file, File};
    use std::io::Write;

    const FILE_PATH: &str = "./config-test.yaml";
    fn create_config_file() -> Result<(), Box<dyn Error>> {
        let config = Config {
            env: "dev".to_string(),
            api_key_data: Default::default(),
            auth0_data: Default::default(),
            mongo_uri: "http://test.com".to_string(),
        };

        // Serialize the struct to YAML
        let yaml_result = serde_yaml::to_string(&config);

        match yaml_result {
            Ok(yaml_string) => match File::create(FILE_PATH) {
                Ok(mut file) => match file.write_all(yaml_string.as_bytes()) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(Box::try_from(e).unwrap()),
                },
                Err(e) => Err(Box::try_from(e).unwrap()),
            },
            Err(e) => Err(Box::try_from(e).unwrap()),
        }
    }

    fn delete_config_file() {
        remove_file(FILE_PATH).expect("Couldn't delete test file");
    }

    // Test case for creating config file, and reading that again
    #[test]
    fn test_deserialize_yaml() {
        let result = create_config_file();
        assert!(!result.is_err());
        let config_result = load(FILE_PATH);
        assert!(!config_result.is_err());
        let config = config_result.unwrap();
        assert_eq!(config.env, "dev");
        assert_eq!(config.mongo_uri, "http://test.com");
        delete_config_file();
    }
}
