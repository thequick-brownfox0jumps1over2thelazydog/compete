# Compete

## Rust

### 実装
- 数値型は考えている時間が勿体ないので基本的にisizeに統一、インデックス等でunsignedを強制される箇所のみusizeを指定
- 標準入力で要素数が静的な多次元配列は行をタプルで指定


### cargo-compete

#### Get started
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
cargo compete submit <PROBLEM_NAME> [--no-test]
```

#### 注意
- `compete.toml`
    - `test > toolchain` を現時点でAtCoderでサポートされている `"1.70.0"` に変更
    - `submit > language_id` を現時点でAtCoderにおけるRustの `"5054"` に変更
