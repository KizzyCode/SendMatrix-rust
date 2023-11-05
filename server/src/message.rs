//! A message

/// A message
#[derive(Debug, Clone)]
pub enum Message {
    /// A plaintext message
    Plaintext {
        /// The plaintext to send
        text: Vec<u8>,
    },
    /// A markdown message
    Markdown {
        /// The markdown message to send
        markdown: Vec<u8>,
    },
    /// The raw file to send
    Raw {
        /// The name of the file (without the .raw-extension)
        name: String,
        /// The file contents
        contents: Vec<u8>,
    },
}
