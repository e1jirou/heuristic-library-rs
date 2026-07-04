I am currently participating in an AtCoder Heuristic Contest, and I will use this generative AI as assistance for developing my solution.

When using this generative AI, the "AtCoder Heuristic Contest Generative AI Usage Rules - Version 20250616" apply.
https://info.atcoder.jp/entry/ahc-llm-rules-en

You must not perform any of the following actions:

* Run the solution program.
* Most importantly, you must not run the solution program and then automatically repeat improvements to the approach or code based on the execution results.
* Access social media or YouTube to collect information about the contest.

Here, "solution program" refers to any program created or being created for the purpose of solving this contest problem, regardless of whether it was created by the user or by generative AI, and regardless of whether it is still in progress or already complete.

Compiling the solution program, and giving advice based on execution results, logs, scores, or similar information provided by the user, are not included in the prohibited actions above.

# AGENTS.md

AtCoder Heuristic Contest XXX の問題を解くためのリポジトリです。

## 重要なファイル

- `src/main.rs`: 問題の解答コードを実装するファイル。
- `problem.md`: 問題の内容を記載するファイル。
- `RUNNING.md`: ユーザー向けの実行方法を記載するファイル。

## リポジトリのメンテナンスルール

- `AGENTS.md` と terminal の出力では日本語を使用すること。
- コードのコメントは英語で書くこと。
- Rust ファイルを編集したら、必ず `cargo fmt` と `cargo check` を実行すること。
- 実装する前に、チャットで実装内容を簡潔に説明して確認すること。
- アイディアはなるべく1つずつ実装して、ユーザーから提供された実行結果を確認しながら進めること。
- わからないことがあれば、すぐに質問すること。

## 実装のガイドライン

- （処理が変わらない範囲で）なるべく高速なアルゴリズムを選択すること。
- （計算量のオーダーだけでなく）定数倍も考慮すること。
- 簡潔さや可読性も意識すること。
- 適宜 debug_assert! などを使用して、実装の正しさを確認すること。
- エラーハンドリングにこだわる必要はありません。
