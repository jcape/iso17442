#!/bin/bash

mkdir -p /workspaces/iso17442/.cache/cargo
ln -sf /usr/local/cargo/bin /workspaces/iso17442/.cache/cargo/

rustup toolchain install nightly --profile default

cargo binstall -q -y --force --locked prek
cargo binstall -q -y --force --locked action-validator
cargo binstall -q -y --force --locked cargo-deny
cargo binstall -q -y --force --locked cargo-nono
cargo binstall -q -y --force --locked cargo-nextest
cargo binstall -q -y --force --locked taplo-cli

pushd /workspaces/iso17442 >/dev/null
prek install >/dev/null
popd >/dev/null
