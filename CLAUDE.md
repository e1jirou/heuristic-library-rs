# CLAUDE.md

AtCoder Heuristic Contest の問題を解くためのテンプレートリポジトリです。

## 重要なファイル

- `src/main.rs`: 問題の解答コードを実装するファイル。
- `problem.md`: 問題の内容を記載するファイル。

## リポジトリのメンテナンスルール

- `CLAUDE.md` と terminal の出力では日本語を使用すること。
- コードのコメントは英語で書くこと。
- Rust ファイルを編集したら、必ず `cargo fmt` を実行すること。
- 実装する前に実装内容を terminal に出力して確認すること。

## 実行方法

### 非 interactive 問題

複数のテストケースを並列実行:
```bash
python3 scripts/test_non_interactive.py [solution_name]
```

単一のテストケースを実行:
```bash
cargo run [--release] < tools/in/<case_number>.txt
```

### interactive 問題

複数のテストケースを並列実行:
```bash
python3 scripts/test_interactive.py [solution_name]
```

単一のテストケースを実行 (debug):
```bash
sh scripts/debug_interactive.sh < in/<case_number>.txt
```

単一のテストケースを実行 (release):
```bash
sh scripts/release_interactive.sh < in/<case_number>.txt
```
