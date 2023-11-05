//! The IPC server

use crate::argv::MessageKind;
use std::{
    fs,
    io::{self, Error, ErrorKind, Read},
    path::Path,
};

/// The IPC adapter
#[derive(Debug)]
pub struct Ipc {
    _private: (),
}
impl Ipc {
    /// Sends a message
    pub fn send(ipc_path: &str, kind: MessageKind, payload: String) -> Result<(), Error> {
        match kind {
            MessageKind::Plaintext | MessageKind::Markdown => Self::sendtext(ipc_path, kind, payload),
            MessageKind::Raw => Self::sendraw(ipc_path, payload),
        }
    }

    /// Sends a text message
    fn sendtext(ipc_path: &str, kind: MessageKind, mut payload: String) -> Result<(), Error> {
        // Get the payload
        if payload == "-" {
            // Get stdin
            let mut stdin = io::stdin();
            let mut buf = Vec::new();
            stdin.read_to_end(&mut buf)?;

            // Ensure stdin is UTF-8
            match String::from_utf8(buf) {
                Ok(payload_) => payload = payload_,
                Err(e) => return Err(Error::new(ErrorKind::InvalidData, e)),
            }
        }

        // Write the message to a tempfile
        let uuidname = format!("{}.tmp", Self::uuidgen());
        let tmp = Path::new(ipc_path).join(uuidname);
        fs::write(&tmp, payload)?;

        // Create the final name
        let mut dest = tmp.clone();
        match kind {
            MessageKind::Plaintext => dest.set_extension("txt"),
            MessageKind::Markdown => dest.set_extension("markdown"),
            _ => {
                // Note: `sendtext` should not be called for not text-messages
                #[allow(clippy::unreachable)]
                (unreachable!("`sendtext` should not be called for not text-messages"));
            }
        };

        // Make file persistent
        fs::hard_link(&tmp, dest)?;
        fs::remove_file(tmp)?;
        Ok(())
    }

    /// Sends a raw file message
    fn sendraw(ipc_path: &str, payload: String) -> Result<(), Error> {
        // Get the file name
        let path = Path::new(&payload);
        let Some(filename) = path.file_name() else {
            eprintln!(r#"!> Invalid file path: "{}""#, path.display());
            return Err(Error::from(ErrorKind::InvalidInput));
        };

        // Convert the filename to UTF-8
        let Some(filename) = filename.to_str() else {
            eprintln!(r#"!> Non-UTF-8 filename: "{}""#, path.display());
            return Err(Error::from(ErrorKind::InvalidInput));
        };

        // Ensure that the filename is ascii, otherwise the sendmatrix server will not process it
        let true = filename.is_ascii() else {
            eprintln!(r#"!> Invalid filename: "{}""#, path.display());
            return Err(Error::from(ErrorKind::InvalidInput));
        };

        // Copy the file to a tempfile
        let tmpname = format!("{filename}.tmp");
        let tmp = Path::new(ipc_path).join(tmpname);
        fs::copy(payload, &tmp)?;

        // Create the final name
        let mut dest = tmp.clone();
        dest.set_extension("raw");

        // Make the file permanent
        fs::hard_link(&tmp, dest)?;
        fs::remove_file(tmp)?;
        Ok(())
    }

    /// Generates a new UUID
    fn uuidgen() -> String {
        // Generate 16 random bytes
        let mut bytes = [0; 16];
        // Note: If getrandom does not work we want to terminate
        #[allow(clippy::expect_used)]
        getrandom::getrandom(&mut bytes).expect("failed to generate UUID");

        // Format UUID
        #[rustfmt::skip]
        format! {
            "{:02X}{:02X}{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            bytes[0x0], bytes[0x1], bytes[0x2], bytes[0x3],
            bytes[0x4], bytes[0x5],
            bytes[0x6], bytes[0x7],
            bytes[0x8], bytes[0x9],
            bytes[0xA], bytes[0xB],bytes[0xC], bytes[0xD],bytes[0xE], bytes[0xF],
        }
    }
}
