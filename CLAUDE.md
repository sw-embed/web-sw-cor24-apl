# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project: web-sw-cor24-apl -- Browser-based APL Environment on COR24

Browser-based APL environment (REPL/editor) running the sw-cor24-apl
interpreter on the COR24 emulator via WASM. Features ASCII surface
syntax with lowercase reserved words (rho, iota, take, drop) and
uppercase user identifiers. Includes APL glyph prettification in the
editor (display rho as the APL symbol but store canonical ASCII).

## Related Projects

All COR24 repos live under `~/github/sw-embed/` as siblings:

- `sw-cor24-apl` -- APL interpreter (C, targeting COR24)
- `sw-cor24-emulator` -- COR24 assembler and emulator (Rust)
- `sw-cor24-assembler` -- COR24 assembler library
- `web-sw-cor24-pcode` -- P-code VM debugger (browser, closest reference)
- `web-sw-cor24-assembler` -- COR24 assembly IDE (browser)
- `sw-cor24-pcode` -- P-code VM, assembler, and linker
- `sw-cor24-rust` -- Rust-to-COR24 pipeline

## CRITICAL: AgentRail Session Protocol (MUST follow exactly)

### 1. START (do this FIRST, before anything else)
```bash
agentrail next
```
Read the output carefully. It contains your current step, prompt,
plan context, and any relevant skills/trajectories.

### 2. BEGIN (immediately after reading the next output)
```bash
agentrail begin
```

### 3. WORK (do what the step prompt says)
Do NOT ask "want me to proceed?". The step prompt IS your instruction.
Execute it directly.

### 4. COMMIT (after the work is done)
Commit your code changes with git. Use `/mw-cp` for the checkpoint
process (pre-commit checks, docs, detailed commit, push).

### 5. COMPLETE (LAST thing, after committing)
```bash
agentrail complete --summary "what you accomplished" \
  --reward 1 \
  --actions "tools and approach used"
```
- If the step failed: `--reward -1 --failure-mode "what went wrong"`
- If the saga is finished: add `--done`

### 6. STOP (after complete, DO NOT continue working)
Do NOT make further code changes after running `agentrail complete`.
Any changes after complete are untracked and invisible to the next
session. Future work belongs in the NEXT step, not this one.

## Key Rules

- **Do NOT skip steps** -- the next session depends on accurate tracking
- **Do NOT ask for permission** -- the step prompt is the instruction
- **Do NOT continue working** after `agentrail complete`
- **Commit before complete** -- always commit first, then record completion

## Useful Commands

```bash
agentrail status          # Current saga state
agentrail history         # All completed steps
agentrail plan            # View the plan
agentrail next            # Current step + context
```

## Build

Edition 2024 for any Rust code. Never suppress warnings.

```bash
trunk build                    # Build WASM to dist/
./scripts/serve.sh             # Dev server
./scripts/build-pages.sh       # Release build to pages/ for GitHub Pages
cargo clippy --all-targets --all-features -- -D warnings  # Lint
cargo fmt --all                # Format
```

## Architecture

- **Trunk** builds the WASM binary and serves it
- **Yew 0.21** CSR framework for the UI (Component trait, Msg enum, html! macro)
- **wasm-bindgen** + **web-sys** for browser APIs
- **cor24-emulator** provides `EmulatorCore` + `Assembler` (path dep to `../sw-cor24-emulator`)
- **sw-cor24-apl** provides the APL interpreter (path dep to `../sw-cor24-apl`)
- **Catppuccin Mocha** dark theme
- **pages/** directory for GitHub Pages deployment
- **build.rs** for compile-time pre-processing and build metadata
- APL prettification layer: display `rho` as `⍴`, `iota` as `⍳`, etc. in the browser

## APL-Specific Context

- ASCII surface syntax: `rho`, `iota`, `take`, `drop` (lowercase reserved words)
- Uppercase user identifiers: `A`, `FOO`, `MATRIX1`
- Context-sensitive display: monadic `rho` = "shape-of", dyadic `A rho B` = "reshape"
- Integer-only, rank <= 2, small arrays
- Right-to-left evaluation (no operator precedence)
- Three display modes: representation (`rho`), shorthand (`⍴`), literate (`shape-of`/`reshape`)

## Available Task Types

- `rust-project-init` -- Scaffold Cargo.toml, src/, Trunk config
- `yew-component` -- Build a Yew Component (html!, Msg enum, update/view)
- `wasm-build` -- Configure wasm-bindgen, web-sys features, Trunk pipeline
- `css-styling` -- Catppuccin Mocha theme, panel layout, responsive design
- `integration` -- Wire path dependencies, bridge APL interpreter to WASM
- `build-script` -- build.rs pre-processing, scripts/serve.sh, scripts/build-pages.sh
- `feature` -- Add UI feature (REPL, editor, output panel, glyph rendering)
- `bug-fix` -- Fix a defect
- `docs` -- Documentation updates

## Key Files (planned)

- `src/lib.rs` -- App component, module declarations
- `src/main.rs` -- Entry point (Yew renderer)
- `src/repl.rs` -- REPL component (input/output, APL evaluation)
- `src/editor.rs` -- Editor component (APL source editing with prettification)
- `src/output.rs` -- Output display component (formatted array output)
- `index.html` -- Entry point with Catppuccin Mocha theme, Trunk directives
- `src/app.css` -- Application styling
- `build.rs` -- Build script (metadata, pre-processing)
- `scripts/serve.sh` -- Dev server script
- `scripts/build-pages.sh` -- Release build to pages/
