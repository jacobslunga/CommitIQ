use std::fs;
use std::path::PathBuf;

pub fn set_api_key(api_key: &str, path: PathBuf) {
    if !path.exists() {
        fs::File::create(&path).expect("Could not create file");
    }

    fs::write(&path, api_key).expect("Could not write to file");
}

pub fn read_api_key_from_file(path: PathBuf) -> Result<String, std::io::Error> {
    if !path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "API key file not found",
        ));
    }

    let api_key = fs::read_to_string(path)?;
    Ok(api_key.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn test_set_api_key() {
        let temp_dir = TempDir::new(".commitiq_test").unwrap();
        let file_path = temp_dir.path().join(".commitiq-test");
        set_api_key("dummy_key", file_path.clone());

        assert!(file_path.exists());

        let contents = fs::read_to_string(&file_path).expect("Could not read .commitiq-test file");
        assert_eq!(contents, "dummy_key");
    }

    #[test]
    fn test_read_api_key_from_file() {
        let temp_dir = TempDir::new(".commitiq_test").unwrap();
        let file_path = temp_dir.path().join(".commitiq-test");
        set_api_key("dummy_key", file_path.clone());

        let api_key =
            read_api_key_from_file(file_path.clone()).expect("Could not read API key from file");

        assert_eq!(api_key, "dummy_key");
    }
}
