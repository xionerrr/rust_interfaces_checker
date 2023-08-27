use std::fs::read_dir;

enum ErrorEnum {
    NotFound,
    OtherError,
}

struct ErrorInfo {
    message: &'static str,
}

impl ErrorEnum {
    fn error_message(&self) -> ErrorInfo {
        match self {
            ErrorEnum::NotFound => ErrorInfo {
                message: "Not found",
            },
            ErrorEnum::OtherError => ErrorInfo {
                message: "Other error",
            },
        }
    }
}

pub fn parse_directories(path: &str) -> eyre::Result<Vec<String>> {
    let mut directories: Vec<String> = Vec::new();
    let default_error = ErrorEnum::NotFound.error_message();
    let other_error = ErrorEnum::OtherError.error_message();

    for entry in read_dir(path)?.filter_map(|e| e.ok()) {
        directories.push(entry.path().to_string_lossy().into_owned())
    }

    if directories.len() <= 0 {
        let error_report = eyre::Report::msg(default_error.message);
        return Err(error_report);
    } else {
        Ok(directories)
    }
}
