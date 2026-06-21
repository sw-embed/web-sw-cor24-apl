# Changelog

## 2026-06-21 — Migrate to cor24-assembler

- Migrated `build.rs` off the removed `cor24_emulator::Assembler` to
  `cor24_assembler::Assembler` (path dep `../sw-cor24-x-assembler`).
- Replaced the `cor24-emulator` `[build-dependencies]` entry with
  `cor24-assembler`; `cor24-emulator` remains a regular dependency for
  `EmulatorCore`.
- Build-time-only change (the assembler pre-assembles the APL prelude in
  `build.rs`); no runtime/WASM behavior change expected.
- Updated `CLAUDE.md`, `docs/plan.md`, and `docs/process.md` which still
  described the assembler as living in `cor24-emulator`.
- Cleared a pre-existing `clippy::unnecessary_sort_by` lint in
  `src/prettify.rs` that was masked while the build was broken, so the
  `clippy -D warnings` gate passes.
- Rebuilt `pages/` for GitHub Pages deployment.
