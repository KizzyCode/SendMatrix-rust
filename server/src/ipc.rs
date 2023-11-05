//! The IPC server

use crate::{config::Config, message::Message};
use std::{
    fs::{self, File},
    io::{Error, ErrorKind, Read},
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

/// The IPC server
#[derive(Debug)]
pub struct IpcServer<'a> {
    /// The config
    config: &'a Config,
    /// The pending IPC messages
    pending: Vec<PathBuf>,
}
impl<'a> IpcServer<'a> {
    /// The file system poll interval
    const POLL_INTERVAL: Duration = Duration::from_secs(3);
    /// The maximum plaintext/markdown message size
    const TEXT_SIZE_MAX: usize = 4096;
    /// The maximum file size for attachments
    const FILE_SIZE_MAX: usize = 2 * 1024 * 1024;

    /// Creates a new server
    pub fn new(config: &'a Config) -> Result<Self, Error> {
        // Initialize self and poll one time to check if everything works as expected
        let mut this = Self { config, pending: Vec::new() };
        let _ = this.has_message()?;

        // Print status and return instance
        eprintln!("*> IPC backlog: {}", this.pending.len());
        Ok(this)
    }

    /// Checks if there are pending messages; scans the IPC directory if necessary
    pub fn has_message(&mut self) -> Result<bool, Error> {
        // Early-abort if we still have some pending messages
        let true = self.pending.is_empty() else {
            return Ok(true);
        };

        // Collect all pending messages; ignore non-file entries, symlinks etc.
        self.pending.clear();
        'read_dir: for maybe_entry in fs::read_dir(&self.config.IPC_PATH)? {
            // Get the entry and ensure it's a file
            let entry = maybe_entry?;
            let true = entry.file_type()?.is_file() else {
                continue 'read_dir;
            };

            // Ignore files with non-ascii names
            let path = entry.path();
            match path.file_name() {
                Some(filename) if filename.is_ascii() => (/* valid */),
                _ => continue 'read_dir,
            };

            // Ignore files that don't end with txt, markdown or raw
            match path.extension() {
                Some(ext) if ext.eq_ignore_ascii_case("txt") => (/* valid */),
                Some(ext) if ext.eq_ignore_ascii_case("markdown") => (/* valid */),
                Some(ext) if ext.eq_ignore_ascii_case("raw") => (/* valid */),
                _ => continue 'read_dir,
            }

            // Store path
            self.pending.push(path);
        }

        // Return if we have pending messages or not
        Ok(!self.pending.is_empty())
    }

    /// Gets the next pending message
    ///
    /// # Important
    /// This function blocks if there is no pending message available
    pub fn next_message(&mut self) -> Result<Message, Error> {
        // Poll until we have messages
        while !self.has_message()? {
            // Yield some time if there are no pending messages
            thread::sleep(Self::POLL_INTERVAL);
        }

        // Get the next messages
        // Note: This is safe since the while-loop above ensures that `self.files` is not empty
        #[allow(clippy::expect_used)]
        let message = self.pending.first().expect("no pending IPC message after successful polling");
        match message.extension() {
            Some(ext) if ext.eq_ignore_ascii_case("txt") => {
                // A .txt-file contains a plaintext message
                let contents = self.read_message(message, Self::TEXT_SIZE_MAX)?;
                Ok(Message::Plaintext { text: contents })
            }
            Some(ext) if ext.eq_ignore_ascii_case("markdown") => {
                // A .markdown-file contains a markdown message
                let contents = self.read_message(message, Self::TEXT_SIZE_MAX)?;
                Ok(Message::Markdown { markdown: contents })
            }
            Some(ext) if ext.eq_ignore_ascii_case("raw") => {
                // A .raw-file is a binary attachment, e.g. `image.jpg.raw` contains the binary attachment `image.jpg`
                // Get the real file name
                let name = match message.file_name() {
                    Some(name) if name.is_ascii() => {
                        // Note: This should be safe because `self.poll_messages` validates the file name
                        #[allow(clippy::expect_used)]
                        let name = name.to_str().expect("invalid file name for pending IPC message");
                        // Note: This is safe since we have checked that the file ends with .raw
                        #[allow(clippy::expect_used)]
                        name.strip_suffix(".raw").expect("cannot strip suffix .raw from string ending with .raw")
                    }
                    _ => {
                        // Note: This should be safe because `self.poll_messages` validates the file name
                        #[allow(clippy::unreachable)]
                        (unreachable!("invalid file name for pending IPC message"))
                    }
                };

                // Get the contents
                let contents = self.read_message(message, Self::FILE_SIZE_MAX)?;
                Ok(Message::Raw { name: name.to_string(), contents })
            }
            _ => {
                // Note: This should be safe because `self.poll_messages` validates the  extensions
                #[allow(clippy::unreachable)]
                (unreachable!("invalid file extension for pending IPC message"))
            }
        }
    }

    /// Marks the currently pending message as completed and removes it from the queue
    pub fn complete_message(&mut self) -> Result<(), Error> {
        // Clear the buffer and delete the currently pending file
        let Some(pending) = self.pending.first() else {
            // Indicate that there was no pending message
            return Err(Error::from(ErrorKind::NotFound));
        };

        // Unlink the file
        fs::remove_file(pending)?;
        // Note: This is safe since we have ensured that the vector has a first element
        self.pending.swap_remove(0);
        Ok(())
    }

    /// Reads an IPC message
    fn read_message(&self, entry: &Path, limit: usize) -> Result<Vec<u8>, Error> {
        // Validate the file size
        let metadata = fs::metadata(entry)?;
        if metadata.len() > limit as u64 {
            // Indicate that the file size is unsupported
            return Err(Error::from(ErrorKind::Unsupported));
        }

        // Open the file
        let len = metadata.len() as usize;
        let mut file = File::open(entry)?;

        // Read the file
        let mut contents = vec![0; len];
        file.read_exact(&mut contents)?;
        Ok(contents)
    }
}
