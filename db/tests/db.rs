use hms_db::{error::HmsDbError, manager::HmsDbManager, models::NewSnip};
use hms_test_utils::{mock_app_dir_client, MockAppDirClient};
use tempfile::TempDir;
use test_case::test_case;

pub fn get_test_manager() -> (TempDir, HmsDbManager<MockAppDirClient>) {
    let (temp_dir, mock_client) = mock_app_dir_client();
    let manager = HmsDbManager::new(mock_client);
    manager.run_pending_migrations().unwrap();
    (temp_dir, manager)
}

#[test]
fn test_insert_snip() {
    let (_temp_dir, manager) = get_test_manager();
    let new_snip = NewSnip::new("alias", "value");

    let snip = manager.with_db(|db| db.insert_snip(&new_snip)).unwrap();

    assert_eq!(new_snip.alias, snip.alias);
    assert_eq!(new_snip.value, snip.value);
    assert_eq!(0, snip.access_count);
}

#[test_case("pickle")]
#[test_case("portal")]
#[test_case("schwifty")]
fn test_case_insensitive_alias_sub_string_search(sub_string: &str) {
    let alias = "A Pickle Portal with Schwifty Casing";
    let (_temp_dir, manager) = get_test_manager();
    let new_snip = NewSnip::new(alias, "value");

    let id = manager.with_db(|db| db.insert_snip(&new_snip)).unwrap().id;
    let snips = &manager
        .with_db(|db| db.find_snip_by_alias(sub_string, 10))
        .unwrap();

    assert_eq!(1, snips.len());
    assert_eq!(id, snips[0].id);
}

#[test]
fn test_alias_length_constraint() {
    let (_temp_dir, manager) = get_test_manager();
    let alias = "ExtremelyLongAndUnnecessarilyComplicatedAliasNameThatDefiesAllLogic";
    let new_snip = NewSnip::new(alias, "value");

    let result = manager.with_db(|db| db.insert_snip(&new_snip));

    if let Err(HmsDbError::QueryError(_)) = result {
        // Correct error type, do nothing
    } else {
        panic!("Expected QueryError, got {:?}", result);
    }
}

#[test]
fn test_rollback_on_query_error() {
    let (_temp_dir, manager) = get_test_manager();
    let new_snip = NewSnip::new("alias", "value");

    let result = manager.with_db(|db| {
        db.insert_snip(&new_snip).unwrap();
        db.insert_snip(&new_snip) // Intentional error (duplicate alias)
    });

    if let Err(HmsDbError::QueryError(_)) = result {
        // Correct error type, do nothing
    } else {
        panic!("Expected QueryError, got {:?}", result);
    }

    let existing = manager
        .with_db(|db| db.find_snip_by_alias("alias", 10))
        .unwrap();

    assert!(existing.is_empty(), "Rollback did not occur as expected");
}
