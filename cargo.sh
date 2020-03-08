#!/bin/bash

export CACHE_DIR=`pwd`/cargo
export CARGO_HOME=$CACHE_DIR
export RUSTUP_HOME=$CACHE_DIR

curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal  --default-toolchain nightly

`pwd`/cargo/bin/cargo build --release
