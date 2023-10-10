# Compete

## Rust

### 実装
- 型
    - 型の明示は時間が勿体ないので基本的に避ける
        - `const` 他、必要な場合もコレクションについては要素の型を `_` として明示を避ける
    - 数値型は考えている時間が勿体ないので基本的に `isize` として定義
        - 要素数（「区別できる」固有データの個数）のみ `usize` として定義
            - `usize` の十分条件は、入力値（ `input!` クロージャ内）のコレクションの要素数か否か
            - 文意から `usize` として定義するべきものもある
        - インデックスに相当する1始まりの入力値は `Usize1` として定義
        - キャストする際は、可能であれば変数定義時点でキャスト
    - 標準入力で要素数が静的な多次元配列は行をタプルで指定
        - 値を参照する際は必ず変数に代入
- 変数・参照
    - 結果を出力する際、ワンライナーで収まる場合は `println!` の引数を変数化せずにそのまま渡す
    - イテレータのリファレンス・デリファレンスは、スコープ内のループ変数の参照時に行う（複数回参照する場合はこの限りでない）


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


### cargo-snippet

#### Get started
```
cargo install cargo-snippet --features="binaries"

cd ${cmp}/rust/snippets
rm $snippet/rust.code-snippets \
  && cargo snippet -t vscode | sed -r 's/"prefix"/"scope": "rust",\n    "prefix"/' > $snippet/rust.code-snippets
```
