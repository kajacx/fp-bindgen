#!/usr/bin/sh

cd simple-protocol && cargo run && cd .. && \ 
cd simple-plugin && cargo build && cd .. && \
cp simple-protocol/bindings/rust-wasmer-runtime/bindings.rs simple-rust-wasmer-runtime/src/spec/bindings.rs && \
cp simple-protocol/bindings/rust-wasmer-runtime/types.rs simple-rust-wasmer-runtime/src/spec/types.rs && \
cp simple-protocol/bindings/rust-wasmer-wasi-runtime/bindings.rs simple-rust-wasmer-runtime/src/wasi_spec/bindings.rs && \
cp simple-protocol/bindings/rust-wasmer-wasi-runtime/types.rs simple-rust-wasmer-runtime/src/wasi_spec/types.rs && \
cd simple-rust-wasmer-runtime && cargo test
