use crate::models::Error;

#[derive(Debug)]
pub struct ErrorInfo {
    message: &'static str,
    status_code: i32,
}

impl Error {
    pub fn error_response(&self) -> ErrorInfo {
        match self {
            Error::NotFound => ErrorInfo {
                message: "Not found",
                status_code: 404,
            },
            Error::Forbidden => ErrorInfo {
                message: "Forbidden",
                status_code: 403,
            },
            Error::OtherError => ErrorInfo {
                message: "Other error",
                status_code: 502,
            },
        }
    }
}
