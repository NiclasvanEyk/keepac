use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy)]
pub enum ErrorExitCode {
    Unknown = 1,
    ChangelogNotFound = 2,
}

#[derive(Debug)]
pub struct KeepacCliError {
    pub message: String,
    pub exit_code: ErrorExitCode,
}

impl Display for KeepacCliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for KeepacCliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<anyhow::Error> for KeepacCliError {
    fn from(value: anyhow::Error) -> Self {
        KeepacCliError {
            message: format!("{}", value),
            exit_code: ErrorExitCode::Unknown,
        }
    }
}

impl From<std::io::Error> for KeepacCliError {
    fn from(value: std::io::Error) -> Self {
        KeepacCliError {
            message: format!("{}", value),
            exit_code: ErrorExitCode::Unknown,
        }
    }
}

impl From<Box<dyn std::error::Error>> for KeepacCliError {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        KeepacCliError {
            message: format!("{}", value),
            exit_code: ErrorExitCode::Unknown,
        }
    }
}
