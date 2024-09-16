#!/bin/sh
cd pkg/rustpushgo
cargo build --release
uniffi-bindgen-go target/release/librustpushgo.dylib --library --out-dir out