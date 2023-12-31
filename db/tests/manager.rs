use hms_db::manager::HmsDbManager;
use hms_test_utils::test_app_dir_client;

#[test]
fn test_db_existence_before_and_after_migration() {
    let (_temp_dir, mock_client) = test_app_dir_client();
    let manager = HmsDbManager::new(mock_client);
    assert!(!manager.db_exists().unwrap());

    manager.run_pending_migrations().unwrap();

    assert!(manager.db_exists().unwrap());
}

#[test]
fn test_pending_migrations_before_and_after_migration() {
    let (_temp_dir, mock_client) = test_app_dir_client();
    let manager = HmsDbManager::new(mock_client);
    assert!(manager.db_has_pending_migrations().unwrap());

    manager.run_pending_migrations().unwrap();

    assert!(!manager.db_has_pending_migrations().unwrap());
}
