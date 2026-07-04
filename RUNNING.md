# 実行方法

このファイルはユーザー向けの実行手順を記載するものです。

## 非 interactive 問題

複数のテストケースを並列実行:
```bash
python3 scripts/test_non_interactive.py [solution_name]
```

単一のテストケースを実行:
```bash
cargo run [--release] < tools/in/<case_number>.txt
```

## interactive 問題

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
