#!/bin/bash

RUST_BACKTRACE=1 RUST_LOG=debug cargo run --example shortcut-remote-remapper -- "$@"