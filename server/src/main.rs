#![doc = include_str!("../README.md")]
// Clippy lints
#![warn(clippy::large_stack_arrays)]
#![warn(clippy::arithmetic_side_effects)]
#![warn(clippy::expect_used)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::indexing_slicing)]
#![warn(clippy::panic)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]
#![warn(clippy::unreachable)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::allow_attributes_without_reason)]
#![warn(clippy::cognitive_complexity)]

mod config;
mod ipc;
mod matrix;
mod message;

use crate::{config::Config, ipc::IpcServer, matrix::Matrix};

fn main() {
    // Load config
    // Note: We use expect here because if we cannot load the config we want to terminate
    #[allow(clippy::expect_used)]
    let config = Config::from_env().expect("failed to load config");
    println!("*> Configuration: `{config:?}`");

    // Create a matrix adapter
    // Note: We use expect here because if we cannot create a matrix adapter we want to terminate
    #[allow(clippy::expect_used)]
    let matrix = Matrix::new(&config).expect("failed to initialize matrix adapter");

    // Create the server and proces messages
    // Note: We use expect here because if we cannot create a server we want to terminate
    #[allow(clippy::expect_used)]
    let mut server = IpcServer::new(&config).expect("failed to start IPC server");
    loop {
        // Get the next message
        // Note: We use expect here because if we cannot read IPC messages we want to terminate
        #[allow(clippy::expect_used)]
        let message = server.next_message().expect("failed to get next IPC message");

        // Send message
        // Note: We use expect here because if we cannot send a message we want to terminate
        #[allow(clippy::expect_used)]
        matrix.send(message).expect("failed to send message");

        // Note: We use expect here because if we cannot process IPC messages we want to terminate
        #[allow(clippy::expect_used)]
        server.complete_message().expect("failed to finalize the processing of the IPC message");
    }
}
