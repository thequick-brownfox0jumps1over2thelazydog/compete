#!/bin/bash

set -eu

# move working path
cd ${cmp}/rust/atcoder

# create new contest workspace
cargo compete new ${1}
cd ${1}

# create Visual Studio Code settings file
mkdir .vscode
echo '{
  "rust-analyzer.check.extraArgs": ["--target-dir=target"],
  "rust-analyzer.linkedProjects": ["Cargo.toml"],
}
' >> .vscode/settings.json

# open Visual Studio Code
code .
