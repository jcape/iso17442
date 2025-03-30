#!/bin/bash

mkdir -p /workspaces/iso17442/.cache/cargo
ln -sf /usr/local/cargo/bin /workspaces/iso17442/.cache/cargo/

cargo install -q cargo-semver-checks
cargo install -q release-plz

pushd /workspaces/iso17442 >/dev/null
pre-commit install >/dev/null
popd >/dev/null

