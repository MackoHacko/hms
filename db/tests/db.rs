use hms_db::{error::HmsDbError, manager::HmsDbManager, models::NewSnip};
use hms_test_utils::{test_app_dir_client, TestAppDirClient};
use test_case::test_case;

pub fn get_test_manager(app_dir_client: &TestAppDirClient) -> HmsDbManager<TestAppDirClient> {
    let manager = HmsDbManager::new(app_dir_client);
    manager.run_pending_migrations().unwrap();
    manager
}

#[test]
fn test_insert_snip() {
    let (_temp_dir, app_dir_client) = test_app_dir_client();
    let manager = get_test_manager(&app_dir_client);
    let new_snip = NewSnip::new("alias", "value");

    let snip = manager.with_db(|db| db.insert_snip(&new_snip)).unwrap();

    assert_eq!(new_snip.alias, snip.alias);
    assert_eq!(new_snip.value, snip.value);
    assert_eq!(0, snip.access_count);
}

#[test]
fn test_insert_snips() {
    let (_temp_dir, app_dir_client) = test_app_dir_client();
    let manager = get_test_manager(&app_dir_client);
    let new_snips = vec![
        NewSnip::new("alias1", "value"),
        NewSnip::new("alias2", "value"),
    ];

    manager.with_db(|db| db.insert_snips(&new_snips)).unwrap();

    let snips = manager
        .with_db(|db| db.find_snips_by_alias(&"alias", 100, 0))
        .unwrap();

    assert_eq!(2, snips.len());
}

#[test_case("pickle")]
#[test_case("portal")]
#[test_case("schwifty")]
fn test_case_insensitive_alias_sub_string_search(sub_string: &str) {
    let alias = "A Pickle Portal with Schwifty Casing";
    let (_temp_dir, app_dir_client) = test_app_dir_client();
    let manager = get_test_manager(&app_dir_client);
    let new_snip = NewSnip::new(alias, "value");

    let id = manager.with_db(|db| db.insert_snip(&new_snip)).unwrap().id;
    let snips = &manager
        .with_db(|db| db.find_snips_by_alias(sub_string, 10, 0))
        .unwrap();

    assert_eq!(1, snips.len());
    assert_eq!(id, snips[0].id);
}

#[test]
fn test_alias_length_constraint() {
    let (_temp_dir, app_dir_client) = test_app_dir_client();
    let manager = get_test_manager(&app_dir_client);
    let alias = "ExtremelyLongAndUnnecessarilyComplicatedAliasNameThatDefiesAllLogic";
    let new_snip = NewSnip::new(alias, "value");

    let result = manager.with_db(|db| db.insert_snip(&new_snip));

    if let Err(HmsDbError::AliasConstraintError(_)) = result {
        // Correct error type, do nothing
    } else {
        panic!("Expected QueryError, got {:?}", result);
    }
}

#[test]
fn test_rollback_on_constraint_error() {
    let (_temp_dir, app_dir_client) = test_app_dir_client();
    let manager = get_test_manager(&app_dir_client);
    let new_snip = NewSnip::new("alias", "value");

    let result = manager.with_db(|db| {
        db.insert_snip(&new_snip).unwrap();
        db.insert_snip(&new_snip) // Intentional error (duplicate alias)
    });

    if let Err(HmsDbError::AliasConstraintError(_)) = result {
        // Correct error type, do nothing
    } else {
        panic!("Expected QueryError, got {:?}", result);
    }

    let existing = manager
        .with_db(|db| db.find_snips_by_alias("alias", 10, 0))
        .unwrap();

    assert!(existing.is_empty(), "Rollback did not occur as expected");
}

#[test]
fn test_increment_access_count() {
    let (_temp_dir, app_dir_client) = test_app_dir_client();
    let manager = get_test_manager(&app_dir_client);
    let new_snip = NewSnip::new("alias", "value");

    let snip = manager.with_db(|db| db.insert_snip(&new_snip)).unwrap();

    assert_eq!(0, snip.access_count);

    let updated_snip = manager
        .with_db(|db| {
            db.increment_snip_access_count(&snip).unwrap();
            db.find_snip_by_id(snip.id)
        })
        .unwrap();

    assert_eq!(1, updated_snip.access_count);
}

#[test]
fn test_most_accessed_snips() {
    let (_temp_dir, app_dir_client) = test_app_dir_client();
    let manager = get_test_manager(&app_dir_client);

    let never_accessed_snip = NewSnip::new("never", "value");
    let least_accessed_snip = NewSnip::new("least", "value");
    let most_accessed_snip = NewSnip::new("most", "value");

    manager
        .with_db(|db| {
            db.insert_snip(&never_accessed_snip).unwrap();
            let least_accessed = db.insert_snip(&least_accessed_snip).unwrap();
            let most_accessed = db.insert_snip(&most_accessed_snip).unwrap();
            db.increment_snip_access_count(&least_accessed).unwrap();
            db.increment_snip_access_count(&most_accessed).unwrap();
            db.increment_snip_access_count(&most_accessed)
        })
        .unwrap();

    let top_list = manager
        .with_db(|db| db.fetch_top_snips_by_access(10))
        .unwrap();

    assert_eq!(2, top_list.len());
    assert_eq!("most", top_list[0].alias);
}
