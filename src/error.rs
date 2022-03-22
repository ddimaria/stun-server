//! Custom errors for this applicatoin.
//!
//! Map errors from libraries to Error.
//!
//! Define a reusable Result type.

use envy::Error as EnvyError;
use log::error;
use std::net::AddrParseError;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("Invalid argument: {0}")]
    Arguments(String),

    #[error("{0}.  Make sure you copied .env.example to .env")]
    Config(String),

    #[error("Error sending the binding request: {0}")]
    BindingRequest(String),

    #[error("Error sending the binding response: {0}")]
    BindingResponse(String),

    #[error("Error decoding: {0}.")]
    Decode(String),

    #[error("Parse error: {0}.")]
    Parse(String),

    #[error("Error receiving bytes: {0}.")]
    Receive(String),

    #[error("Error starting the server: {0}.")]
    Startup(String),
}

// Log out errors
fn log_error(error: Error) -> Error {
    error!("{:?}", error);
    error
}

impl From<EnvyError> for Error {
    fn from(error: EnvyError) -> Self {
        let error = match error {
            EnvyError::MissingValue(error) => format!("Missing config value in .env: {}", error),
            EnvyError::Custom(error) => error,
        };
        log_error(Error::Config(error))
    }
}

impl From<AddrParseError> for Error {
    fn from(error: AddrParseError) -> Self {
        log_error(Error::Parse(error.to_string()))
    }
}
