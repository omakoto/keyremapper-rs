#!/bin/bash

prog_name="${0##*/}" # Remove the directory
prog_name="${prog_name#start-}" # Remove the "start-".
prog_name="${prog_name%.sh}" # Remove the extension.

RUST_BACKTRACE=1 RUST_LOG=debug cargo run --example $prog_name -- "$@"