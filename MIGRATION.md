# Repository split

Extracted from [the original parser repository](https://github.com/traq-markdown-parser/core/tree/131fd3e9be86aa342b29666f9f31602d8b53bda4), including the subsequent TypeScript and renderer separation work.

The core repository preserves the original Git history. The other repositories start with their owned source trees. Package names and Rust module paths are preserved, so moving files does not change generated AST type keys.

On 2026-09-08 this repository was renamed from `trap` to `trap-extension`.
`crates/traq`, `crates/traq-processing`, and the text composition example moved to
[traq](https://github.com/traq-markdown-parser/traq). The original files are preserved
in this repository at commit `bf220c640d23f39e33fa0a52147cdf3bd32998ff`.
The remaining crates provide extension components and do not depend on the traQ distribution.
Rust package names and AST type keys remain unchanged.
