# traP Markdown extensions and traQ presets

traP 固有の Markdown 拡張と traQ 向けの構成を所有します。

| crate | 責務 |
| --- | --- |
| `markdown-trap-contracts` | スタンプ、参照、spoiler、空行のノード契約 |
| `markdown-trap-syntax` | traP 拡張の構文解析 |
| `markdown-trap-text` | traP ノードのテキスト描画 |
| `markdown-trap-extraction` | 参照などの抽出 |
| `markdown-traq` | CommonMark・汎用拡張・traP 拡張を組み合わせた traQ 文法 preset |
| `markdown-traq-processing` | traQ 向けの通知テキスト・参照抽出 preset |

```rust
use markdown_traq::presets;

let parser = presets::traq::v1::parser();
let document = parser.parse("**hello** :stamp:")?;
```

文法と処理は同じ型付き AST を利用します。通知・抽出の使い方は [traq-processing](crates/traq-processing/README.md) を参照してください。

## 開発

```sh
cargo test --locked --workspace --all-features
cargo fmt --all --check
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo run -p markdown-traq-processing --example notification
```

Rust の版は `rust-toolchain.toml` で固定しています。core と commonmark は Git の確定 revision から取得するため、他の checkout は不要です。

## 境界

依存先は [core](https://github.com/traq-markdown-parser/core) と [commonmark](https://github.com/traq-markdown-parser/commonmark) です。traQ preset は下位の文法を選択・構成し、個々のルールを重複実装しません。

Wasm と TypeScript / Go bindings の配布は [sdk](https://github.com/traq-markdown-parser/sdk)、HTML・CSS とアプリケーションの表示方針は [traq-markdown-it](https://github.com/traPtitech/traq-markdown-it) が担当します。
