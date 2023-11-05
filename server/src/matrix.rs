//! A outgoing adapter for matrix

use crate::{config::Config, message::Message};
use std::{
    io::{Error, ErrorKind, Write},
    process::{Command, Stdio},
};

/// The matrix adapter
pub struct Matrix<'a> {
    /// The config
    config: &'a Config,
}
impl<'a> Matrix<'a> {
    /// Creates a new matrix adapter
    pub fn new(config: &'a Config) -> Result<Self, Error> {
        // Init self and get username to ensure matrix commander exists and is configured
        let this = Self { config };
        let username = this.matrix_commander(&["--whoami"], b"")?;

        // Print status and return instance
        eprintln!("*> User: `{}`", username.trim_end());
        Ok(this)
    }

    /// Sends a message to the matrix-commander's configured default room
    pub fn send(&self, message: Message) -> Result<(), Error> {
        // Prepare message
        let (args, data) = match message {
            Message::Plaintext { text } => (vec!["--message", "-"], text),
            Message::Markdown { markdown } => (vec!["--message", "-", "--markdown"], markdown),
            Message::Raw { ref name, contents } => (vec!["--file", "-", "--file-name", name], contents),
        };

        // Send message
        self.matrix_commander(&args, &data)?;
        Ok(())
    }

    /// Executes a matrix commander command
    fn matrix_commander(&self, args: &[&str], data: &[u8]) -> Result<String, Error> {
        // Start the matrix commander
        let mut matrix_commander = Command::new(&self.config.MATRIX_PATH)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        // Write all data to stdin and close it
        // Note: Since we create a dedicated pipe for stdin, this should never fail
        #[allow(clippy::expect_used)]
        let mut stdin = matrix_commander.stdin.take().expect("failed to get stdin from spawned child process");
        stdin.write_all(data)?;
        drop(stdin);

        // Wait for matrix commander to complete
        let result = matrix_commander.wait_with_output()?;
        let true = result.status.success() else {
            // Signalize that matrix commander failed
            return Err(Error::from(ErrorKind::Other));
        };

        // Return stdout
        match String::from_utf8(result.stdout) {
            Ok(stdout) => Ok(stdout),
            Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
        }
    }
}
