#!/bin/sh
set -e

# Delete existing docs
rm -rf doc

# Generate new docs
cd compiler
cargo doc

# Copy docs from excluded folder to visible folder
cd ..
cp -r compiler/target/doc doc
