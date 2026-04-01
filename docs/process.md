# Development Process

## Overview

Development workflow for web-sw-cor24-apl, a browser-based APL
environment built with Yew/Rust/WASM on the COR24 emulator.

## Build and Test

```bash
trunk build                                                  # dev build
./scripts/serve.sh                                           # dev server (port 9957)
./scripts/build-pages.sh                                     # release build to pages/
cargo clippy --all-targets --all-features -- -D warnings     # lint (zero warnings)
cargo fmt --all                                              # format
```

## Pre-Commit Quality Gates

All changes must pass before committing:

1. `cargo clippy --all-targets --all-features -- -D warnings` -- zero warnings
2. `cargo fmt --all` -- all code formatted
3. `markdown-checker -f "**/*.md"` -- ASCII-only markdown
4. `sw-checklist` -- project standards compliance
5. `trunk build` -- WASM build succeeds

Use `/mw-cp` for the full checkpoint process (pre-commit checks,
documentation updates, detailed commit, and push).

## Commit Workflow

- Direct commits to `main` (single developer)
- Clear, descriptive commit messages with co-authorship
- Push immediately after commit
- Use `agentrail` for step tracking when working in sagas

## Code Standards

- Rust edition 2024
- Zero clippy warnings (never suppress with `#[allow(...)]`)
- Inline format arguments: `format!("{name}")` not `format!("{}", name)`
- Module docs with `//!`, item docs with `///`
- Files under 500 lines, functions under 50 lines

## Deployment

GitHub Pages from `pages/` directory:

```bash
./scripts/build-pages.sh    # builds to pages/
git add pages/
git commit -m "deploy: update GitHub Pages"
git push
```

URL: https://sw-embed.github.io/web-sw-cor24-apl/

## Dependencies

Path dependencies to sibling repos under `~/github/sw-embed/`:

- `cor24-emulator` -- COR24 assembler and emulator
- `cor24-isa` -- instruction set architecture definitions
- `sw-cor24-apl` -- APL interpreter (C source, assembled at build time)
