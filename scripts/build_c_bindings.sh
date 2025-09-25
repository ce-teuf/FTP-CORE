#!/bin/bash
cd crates/ftp_core_bindings_c
cargo build --release
./copy_artifacts.sh