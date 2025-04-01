#!/bin/bash
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk serve
