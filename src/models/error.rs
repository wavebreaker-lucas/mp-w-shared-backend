use std::fmt;
use windows::core;

#[derive(Debug)]
pub enum Error {
    AutomationError(core::Error),
    EmitError(String),
    ElementError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::AutomationError(e) => write!(f, "Automation error: {}", e),
            Error::EmitError(e) => write!(f, "Event emission error: {}", e),
            Error::ElementError(e) => write!(f, "Element error: {}", e),
        }
    }
}

// Keep the From implementation we added earlier
impl From<core::Error> for Error {
    fn from(error: core::Error) -> Self {
        Error::AutomationError(error)
    }
}