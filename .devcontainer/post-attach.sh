#!/bin/bash

mkdir -p /workspaces/iso17442/.cache/cargo
ln -sf /usr/local/cargo/bin /workspaces/iso17442/.cache/cargo/

RUSTC_WRAPPER_save="$RUSTC_WRAPPER"
unset RUSTC_WRAPPER
cargo binstall -q -y --force sccache
export RUSTC_WRAPPER="$RUST_WRAPPER_save"

cargo binstall -q -y --force cargo-semver-checks
cargo binstall -q -y --force release-plz

pushd /workspaces/iso17442 >/dev/null
pre-commit install >/dev/null
popd >/dev/null
