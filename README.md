# utils

Rust の学習用に、Unix の標準コマンドを自分で再実装した CLI ツール集です。

## 目的

- Rust の言語仕様・標準ライブラリに手を動かしながら慣れる
- ファイル I/O、パス操作、エラーハンドリング、プロセスの終了コードといった
  「小さいけれど実用的な CLI」を作るのに必要な要素を一通り経験する
- 既存のコマンドの仕様を読み解き、少しずつ再現していく

## 方針

- **標準ライブラリのみで実装する。** 学習が目的なので、外部クレートは原則として追加しない。
  CLI の引数解析も自前で書く（`clap` などの導入は必要になったら改めて検討する）。
- **1 ツール = 1 クレート。** 各ツールは `crates/<tool-name>/` 以下に独立したクレートとして置き、
  ルートの Cargo ワークスペースでまとめて管理する。
- **システムのコマンドと名前を衝突させない。** 実装したコマンドには `r` を付けた名前を使う
  （`ls` → `lsr`）。

## 収録ツール

| クレート | バイナリ | 対応するコマンド | 状態 |
| --- | --- | --- | --- |
| [`crates/lsr`](crates/lsr) | `lsr` | `ls` | 雛形のみ |
| [`crates/common`](crates/common) | （ライブラリ） | — | 空。共通ロジックが出てきたら切り出す |

## 使い方

```console
# ワークスペース全体をビルドする
cargo build --workspace

# 個別のツールを実行する
cargo run -p lsr

# テスト・Lint
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
```

## ライセンス

以下のいずれかを選択して利用できます。

- Apache License, Version 2.0 （[`LICENSE-APACHE`](LICENSE-APACHE)）
- MIT License （[`LICENSE-MIT`](LICENSE-MIT)）

意図的に別段の表明をしない限り、このリポジトリへの貢献は上記のデュアルライセンスの下で
提供されるものとします。
