use hms_config::{error::HmsConfigError, manager::HmsConfigManager, models::HmsConfig};
use hms_test_utils::{test_app_dir_client, TestAppDirClient};
use std::path::PathBuf;

#[test]
fn test_save_exists() {
    let (_temp_dir, app_dir_client) = test_app_dir_client();
    let manager = HmsConfigManager::new(&app_dir_client);
    let config = HmsConfig::default();

    assert!(!manager.config_exists().unwrap());
    manager.save_config(&config).unwrap();
    assert!(manager.config_exists().unwrap());
}

#[test]
fn test_save_load() {
    let (_temp_dir, app_dir_client) = test_app_dir_client();
    let manager = HmsConfigManager::new(&app_dir_client);
    let config = HmsConfig::default();

    manager.save_config(&config).unwrap();
    assert!(manager.load_config().is_ok());
}

#[test]
fn test_update_snip_limit() {
    let (_temp_dir, app_dir_client) = test_app_dir_client();
    let manager = HmsConfigManager::new(&app_dir_client);
    let config = HmsConfig::default();
    manager.save_config(&config).unwrap();
    manager.update_snip_limit(20).unwrap();
    let updated_config = manager.load_config().unwrap();
    assert_eq!(20, updated_config.snip_limit);
}

#[test]
fn test_error_on_save() {
    let mock_path = PathBuf::from("/non/existent/path");
    let mock_client = TestAppDirClient {
        app_dir_path: mock_path,
    };
    let manager = HmsConfigManager::new(&mock_client);
    let config = HmsConfig::default();
    match manager.save_config(&config) {
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
    let mock_client = TestAppDirClient {
        app_dir_path: mock_path,
    };
    let manager = HmsConfigManager::new(&mock_client);
    match manager.load_config() {
        Err(e) => match e {
            HmsConfigError::IO(_) => (),
            _ => panic!("Unexpected error type"),
        },
        Ok(_) => panic!("Oh noes, unexpected success"),
    }
}