#!/bin/bash

cargo build --target wasm32-wasi
wasm-tools component new --output ../components/actor.component.wasm --adapt ../../wasi_snapshot_preview1.wasm ../../target/wasm32-wasi/debug/actor.wasm
