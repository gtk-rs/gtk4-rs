#!/bin/sh

export MESON_SOURCE_ROOT="$1"
export CARGO_TARGET_DIR="$2"

cargo build --manifest-path "$MESON_SOURCE_ROOT"/Cargo.toml

cp -r "$CARGO_TARGET_DIR"/"debug"/* "examples"/
