# Compete

## Rust

### cargo-compete

#### Get started
```
cargo install cargo-compete

mkdir -p ./rust/atcoder && cd ./rust/atcoder
cargo compete login atcoder
# 2を選択
cargo compete init atcoder

# CONTEST_NAME: practice, abc123, etc.
cargo compete new <CONTEST_NAME>

# PROBLEM_NAME: a, b, c, ...
cargo compete test <PROBLEM_NAME>
cargo compete submit <PROBLEM_NAME>
```

#### 注意
- `compete.toml`
    - `test > toolchain` を現時点でAtCoderでサポートされている `"1.70.0"` に変更
    - `submit > language_id` を現時点でAtCoderにおけるRustの `"5054"` に変更
