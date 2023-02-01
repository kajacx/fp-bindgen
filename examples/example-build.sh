#!/usr/bin/sh

echo "Generating protocol..." && \
cd example-protocol && cargo run && cd .. && \
echo "Protocol generated, building plugin..." && \
cd example-plugin && cargo build && cd .. && \
#echo "Plugin built, copying files..." && \
#cp example-protocol/bindings/rust-wasmer-runtime/bindings.rs example-rust-wasmer-runtime/src/spec/bindings.rs && \
#cp example-protocol/bindings/rust-wasmer-runtime/types.rs example-rust-wasmer-runtime/src/spec/types.rs && \
#cp example-protocol/bindings/rust-wasmer-wasi-runtime/bindings.rs example-rust-wasmer-runtime/src/wasi_spec/bindings.rs && \
#cp example-protocol/bindings/rust-wasmer-wasi-runtime/types.rs example-rust-wasmer-runtime/src/wasi_spec/types.rs && \
echo "Files copied, running runtime test..." && \
cd example-rust-wasmer-runtime && cargo test && \
echo "All done."
