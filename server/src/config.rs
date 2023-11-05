//! The server configuration

use std::{
    env::{self, VarError},
    io::{Error, ErrorKind},
    str::FromStr,
};

/// The server configuration
#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct Config {
    /// The path to the IPC directory
    pub IPC_PATH: String,
    /// The path to the matrix commander binary
    pub MATRIX_PATH: String,
}
impl Config {
    /// Loads the config from environment
    pub fn from_env() -> Result<Self, Error> {
        Ok(Self {
            IPC_PATH: Self::get_or("IPC_PATH", "/var/run/sendmatrix")?,
            MATRIX_PATH: Self::get_or("MATRIX_PATH", "/usr/bin/matrix-commander-rs")?,
        })
    }

    /// Gets a variable from the environment and parses it
    pub fn get_or<T, D>(name: &str, default: D) -> Result<T, Error>
    where
        T: From<D>,
        T: FromStr,
        T::Err: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        // Get the value from environment
        let value = match env::var(name) {
            Ok(value) => value,
            Err(VarError::NotPresent) => return Ok(T::from(default)),
            Err(e) => return Err(Error::new(ErrorKind::InvalidData, e)),
        };

        // Parse the value
        match value.parse() {
            Ok(parsed) => Ok(parsed),
            Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
        }
    }
}
