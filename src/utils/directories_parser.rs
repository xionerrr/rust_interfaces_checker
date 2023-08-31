use std::fs::read_dir;

use crate::models::Error;

pub fn parse_directories(path: &str) -> eyre::Result<Vec<String>> {
    let mut directories: Vec<String> = Vec::new();

    for entry in read_dir(path)?.filter_map(|e| e.ok()) {
        directories.push(entry.path().to_string_lossy().into_owned());
    }

    if directories.len() <= 0 {
        let error: crate::utils::ErrorInfo = Error::error_response(&Error::NotFound);
        println!(" test {:?}", error);
        let error_report = eyre::Report::msg("test");
        return Err(error_report);
    } else {
        Ok(directories)
    }
}
