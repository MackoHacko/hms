use hms_common::app_dir_client::AppDirClient;
use std::{io::Error, path::PathBuf};
use tempfile::TempDir;

pub struct MockAppDirClient {
    pub mock_path: PathBuf,
}

impl AppDirClient for MockAppDirClient {
    fn get_app_dir_path(&self) -> Result<PathBuf, Error> {
        Ok(self.mock_path.clone())
    }
}

pub fn mock_app_dir_client() -> (TempDir, MockAppDirClient) {
    let temp_dir = tempfile::tempdir().unwrap();
    let mock_path = temp_dir.path().to_owned();
    let mock_client = MockAppDirClient { mock_path };
    (temp_dir, mock_client)
}
