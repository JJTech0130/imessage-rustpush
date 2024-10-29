#!/bin/sh
set -e
cd pkg/rustpushgo
cargo fmt
cargo build --release

# Check if the uniffi-bindgen-go binary is installed, if not install it
if ! command -v uniffi-bindgen-go &> /dev/null
then
    cargo install uniffi-bindgen-go --git https://github.com/NordSecurity/uniffi-bindgen-go --tag v0.2.1+v0.25.0
fi
uniffi-bindgen-go target/release/librustpushgo.a --library --out-dir ..
