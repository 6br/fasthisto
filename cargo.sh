#!/bin/bash 

export CACHE_DIR=`pwd`/cargo
export CARGO_HOME=$CACHE_DIR
export RUSTUP_HOME=$CACHE_DIR

curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- --no-modify-path -y --profile minimal --default-toolchain nightly

mkdir .cargo

target=`rustup show active-toolchain`

if [[ ${target} =~ ^[^-]*-(.*)\ .*$ ]]; then
  echo -e "[target.${BASH_REMATCH[1]}]\nlinker = '$cc'" > .cargo/config
fi


`pwd`/cargo/bin/cargo build --release # C-compiler, library path, 

