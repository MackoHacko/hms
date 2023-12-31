use hms_common::app_dir_client::AppDirClient;
use std::{io::Error, path::PathBuf};
use tempfile::TempDir;

pub struct TestAppDirClient {
    pub app_dir_path: PathBuf,
}

impl AppDirClient for TestAppDirClient {
    fn get_app_dir_path(&self) -> Result<PathBuf, Error> {
        Ok(self.app_dir_path.clone())
    }
}

pub fn test_app_dir_client() -> (TempDir, TestAppDirClient) {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path().to_owned();
    let mock_client = TestAppDirClient {
        app_dir_path: temp_path,
    };
    (temp_dir, mock_client)
}
