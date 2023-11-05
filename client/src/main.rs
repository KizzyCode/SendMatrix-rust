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

mod argv;
mod ipc;

use crate::argv::Argv;
use ipc::Ipc;

fn main() {
    // Note: If the argv-parsing fails, we want to terminate
    #[allow(clippy::expect_used)]
    let Argv { ipc_path, kind, payload } = Argv::load().expect("failed to parse argv");

    // Note: If the IPC message sending fails, we want to terminate
    #[allow(clippy::expect_used)]
    Ipc::send(&ipc_path, kind, payload).expect("failed to send IPC message");
}
