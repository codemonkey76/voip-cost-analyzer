use std::{error::Error, fmt, io};

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    IoError(io::Error),
    CsvError(csv::Error),
    XlsxError(calamine::XlsxError),
    ParseError(chrono::format::ParseError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "IO Error: {}", e),
            AppError::CsvError(e) => write!(f, "CSV Error: {}", e),
            AppError::XlsxError(e) => write!(f, "Xls Error: {}", e),
            AppError::ParseError(e) => write!(f, "Parse Error: {}", e),
        }
    }
}

impl Error for AppError {}

impl From<chrono::format::ParseError> for AppError {
    fn from(error: chrono::format::ParseError) -> Self {
        AppError::ParseError(error)
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::IoError(error)
    }
}

impl From<csv::Error> for AppError {
    fn from(error: csv::Error) -> Self {
        AppError::CsvError(error)
    }
}

impl From<calamine::XlsxError> for AppError {
    fn from(error: calamine::XlsxError) -> Self {
        AppError::XlsxError(error)
    }
}
