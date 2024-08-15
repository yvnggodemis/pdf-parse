#!/bin/bash
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu

cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu

upx -9 target/x86_64-unknown-linux-gnu/release/pdfsearcher
upx -9 target/x86_64-pc-windows-gnu/release/pdfsearcher.exe