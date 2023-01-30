#!/usr/bin/sh

cd example-protocol && cargo run && cd .. && \
cd example-plugin && cargo build && cd .. && \
cp example-protocol/bindings/rust-wasmer-runtime/bindings.rs example-rust-wasmer-runtime/src/spec/bindings.rs && \
cp example-protocol/bindings/rust-wasmer-runtime/types.rs example-rust-wasmer-runtime/src/spec/types.rs && \
cp example-protocol/bindings/rust-wasmer-wasi-runtime/bindings.rs example-rust-wasmer-runtime/src/wasi_spec/bindings.rs && \
cp example-protocol/bindings/rust-wasmer-wasi-runtime/types.rs example-rust-wasmer-runtime/src/wasi_spec/types.rs && \
cd example-rust-wasmer-runtime && cargo test
