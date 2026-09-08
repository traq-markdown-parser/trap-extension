# traP Markdown extensions

traP 固有の Markdown 拡張部品を提供します。traQ 向けの組み合わせと配布は
[traq](https://github.com/traq-markdown-parser/traq) が所有します。

| crate | 責務 |
| --- | --- |
| `markdown-trap-contracts` | スタンプ、参照、spoiler、空行のノード契約 |
| `markdown-trap-syntax` | traP 拡張の構文解析 |
| `markdown-trap-text` | traP ノードのテキスト描画 |
| `markdown-trap-extraction` | 参照などの抽出 |

各ルールの Plugin を、利用側の GrammarBuilder / PresetBuilder に追加して使います。
文法の選択と順序、通知 URL の表示方針、処理結果の組み合わせは利用側で決めます。
[traQ の構成・実行例](https://github.com/traq-markdown-parser/traq/tree/main/crates/traq-processing)
を参照してください。

## 開発

```sh
cargo test --locked --workspace --all-features
cargo fmt --all --check
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
```

Rust の版は `rust-toolchain.toml` で固定しています。依存する
[core](https://github.com/traq-markdown-parser/core) と
[commonmark](https://github.com/traq-markdown-parser/commonmark) は Git の確定 revision
から取得するため、他の checkout は不要です。traQ の構成・配布には依存しません。

リポジトリ名は `trap` から `trap-extension` に変更しました。Rust package 名と
ノード型の通信キーは維持しています。配置変更の履歴は [MIGRATION.md](MIGRATION.md)
を参照してください。
