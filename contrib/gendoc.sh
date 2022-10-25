#!/bin/sh
set -e

cd compiler
cargo doc

cd ..
cp -r compiler/target/doc doc
