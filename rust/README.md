# Compete > Rust

# cargo-compete
```
cargo install cargo-compete

mkdir -p ./rust/atcoder && cd ./rust/atcoder
cargo compete login atcoder
# 2を選択
cargo compete init atcoder

cd ${cmp}/rust/atcoder
# CONTEST_NAME: practice, abc123, etc.
./new.sh <CONTEST_NAME>

# PROBLEM_NAME: a, b, c, ...
cargo compete test <PROBLEM_NAME>
cargo compete submit --release <PROBLEM_NAME> [--no-test]
```
- `compete.toml`
    - `test > toolchain` を現時点でAtCoderでサポートされている `"1.70.0"` に変更
    - `submit > language_id` を現時点でAtCoderにおけるRustの `"5054"` に変更


# cargo-snippet
```
cargo install cargo-snippet --features="binaries"

cd rust/snippets
cargo snippet -t vscode | sed -r 's/"prefix"/"scope": "rust",\n    "prefix"/' > rust.code-snippets
