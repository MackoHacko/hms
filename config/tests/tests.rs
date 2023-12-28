use hms_common::app_dir_client::AppDirClient;
use hms_config::{error::HmsConfigError, HmsConfig};
use std::{io::Error, path::PathBuf};
use tempfile::TempDir;

struct MockAppDirClient {
    pub mock_path: PathBuf,
}

impl AppDirClient for MockAppDirClient {
    fn get_app_dir_path(&self) -> Result<PathBuf, Error> {
        Ok(self.mock_path.clone())
    }
}

fn setup_mock_client() -> (TempDir, MockAppDirClient) {
    let temp_dir = tempfile::tempdir().unwrap();
    let mock_path = temp_dir.path().to_owned();
    let mock_client = MockAppDirClient { mock_path };
    (temp_dir, mock_client)
}

#[test]
fn test_save_exists() {
    let (_temp_dir, mock_client) = setup_mock_client();
    let config = HmsConfig::default();

    assert!(!HmsConfig::exists(&mock_client).unwrap());
    config.save(&mock_client).unwrap();
    assert!(HmsConfig::exists(&mock_client).unwrap());
}

#[test]
fn test_save_load() {
    let (_temp_dir, mock_client) = setup_mock_client();
    let config = HmsConfig::default();

    config.save(&mock_client).unwrap();
    assert!(HmsConfig::load(&mock_client).is_ok());
}

#[test]
fn test_update_snip_limit() {
    let (_temp_dir, mock_client) = setup_mock_client();
    let config = HmsConfig::default();

    config.update_snip_limit(&mock_client, 20).unwrap();
    let updated_config = HmsConfig::load(&mock_client).unwrap();
    assert_eq!(updated_config.snip_limit, 20);
}

#[test]
fn test_error_on_save() {
    let mock_path = PathBuf::from("/non/existent/path");
    let mock_client = MockAppDirClient { mock_path };
    let config = HmsConfig::default();
    match config.save(&mock_client) {
        Err(e) => match e {
            HmsConfigError::IO(_) => (),
            _ => panic!("Unexpected error type"),
        },
        Ok(_) => panic!("Oh noes, unexpected success"),
    }
}

#[test]
fn test_error_on_load() {
    let mock_path = PathBuf::from("/non/existent/path");
    let mock_client = MockAppDirClient { mock_path };
    match HmsConfig::load(&mock_client) {
        Err(e) => match e {
            HmsConfigError::IO(_) => (),
            _ => panic!("Unexpected error type"),
        },
        Ok(_) => panic!("Oh noes, unexpected success"),
    }
}
