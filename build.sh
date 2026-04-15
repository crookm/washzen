#!/bin/sh

curl https://sh.rustup.rs -sSf | sh -s -- -y
. "$HOME/.cargo/env"

cargo install trunk
rustup target add wasm32-unknown-unknown

trunk build --release