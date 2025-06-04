#!/bin/bash
RUST_LOG=info RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk serve
