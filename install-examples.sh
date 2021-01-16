#!/bin/sh

cd "${0%/*}"

cargo install --path . --examples
