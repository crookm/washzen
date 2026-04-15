#!/bin/sh

curl https://sh.rustup.rs -sSf | sh -s -- -y
cargo install trunk

trunk build --release