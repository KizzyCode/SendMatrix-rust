[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/SendMatrix-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/SendMatrix-rust)


# `sendmatrix-server`
Welcome to `sendmatrix-server` 🎉
`sendmatrix-server` is the server application for `sendmatrix`.
`sendmatrix` is a tiny utility to send messages over matrix in automatized contexts. It works as a wrapper around
[`matrix-commander-rs`](https://crates.io/crates/matrix-commander) and offers a file-based IPC mechanism which is
docker-friendly and can be used via the `sendmatrix` utility, but also via simple shells scripts etc.


## Example
```sh
# Configure IPC path and `matrix-commander-rs` location
export IPC_PATH=../ipc
export MATRIX_PATH=$HOME/.cargo/bin/matrix-commander-rs

# Start the server
sendmatrix-server
```
