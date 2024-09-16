#!/bin/sh
cd pkg/rustpushgo
cargo build --release
uniffi-bindgen-go target/release/librustpushgo.a --library --out-dir out