#!/bin/sh

cd `dirname "$0"`

cargo +nightly build --package state_of_godot_and_the_web_2025_gdextension &&
cargo +nightly build --package state_of_godot_and_the_web_2025_gdextension \
	--features godot/experimental-wasm,godot/experimental-wasm-nothreads,godot/lazy-function-tables \
	--target wasm32-unknown-emscripten -Zbuild-std $@

