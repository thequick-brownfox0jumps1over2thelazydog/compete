#!/bin/bash

set -eu

# create new contest workspace
cd ${cmp}/rust/atcoder
cargo compete new ${1}

# update Visual Studio Code settings file
cd ${cmp}/.vscode
jq \
  --arg CONTEST ${1} \
  '."rust-analyzer.linkedProjects" |= (.+["rust/atcoder/" + $CONTEST + "/Cargo.toml"] | unique)' \
  settings.json \
  1<> settings.json
