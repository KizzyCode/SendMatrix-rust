//! A simple argv parser

use std::{
    collections::HashMap,
    env,
    io::{Error, ErrorKind},
};

/// The message kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageKind {
    /// A plaintext message
    Plaintext,
    /// A markdown message
    Markdown,
    /// A raw message/attachment
    Raw,
}
impl TryFrom<String> for MessageKind {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "plaintext" | "text" => Ok(MessageKind::Plaintext),
            "markdown" => Ok(MessageKind::Markdown),
            "raw" => Ok(MessageKind::Raw),
            _ => Err(Error::from(ErrorKind::InvalidInput)),
        }
    }
}

/// The argv predigested to a usable format
#[derive(Debug, Clone)]
pub struct Argv {
    /// The path to the IPC directory
    pub ipc_path: String,
    /// The message kind
    pub kind: MessageKind,
    /// The message payload
    pub payload: String,
}
impl Argv {
    /// The valid argument keys
    const VALID_KEYS: &[&'static str] = &["ipc-path", "type", "payload"];

    /// Loads the argv and predigests them
    pub fn load() -> Result<Self, Error> {
        // Ingest argv
        let mut argv = Self::ingest_argv()?;

        // Get the raw argument values or choose a default value
        let ipc_path = argv.remove("ipc-path").unwrap_or_else(|| String::from("/var/run/sendmatrix"));
        let type_ = argv.remove("type").unwrap_or_else(|| String::from("plaintext"));
        let payload = argv.remove("payload").unwrap_or_else(|| String::from("-"));

        // Init self
        let kind = MessageKind::try_from(type_)?;
        Ok(Self { ipc_path, kind, payload })
    }

    /// Loads all valid argv into a key-value map
    fn ingest_argv() -> Result<HashMap<String, String>, Error> {
        // Parse all arguments as key-value pairs
        let mut argv = HashMap::new();
        for arg in env::args().skip(1) {
            // Split the argument into key-value
            let Some((key, value)) = arg.split_once('=') else {
                eprintln!("!> Invalid argument: {arg}");
                return Err(Error::from(ErrorKind::InvalidInput));
            };

            // Remove the "--"-argument prefix
            let Some(key) = key.strip_prefix("--") else {
                eprintln!("!> Invalid key: {key}");
                return Err(Error::from(ErrorKind::InvalidInput));
            };

            // Ensure that the key is valid
            let true = Self::VALID_KEYS.contains(&key) else {
                eprintln!("!> Unknown key: {key}");
                return Err(Error::from(ErrorKind::InvalidInput));
            };

            // Register pair
            let None = argv.insert(key.to_string(), value.to_string()) else {
                eprintln!("!> Duplicated key: {key}");
                return Err(Error::from(ErrorKind::InvalidInput));
            };
        }
        Ok(argv)
    }
}
