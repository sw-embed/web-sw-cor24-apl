# web-sw-cor24-apl

Browser-based APL environment running the
[sw-cor24-apl](https://github.com/sw-embed/sw-cor24-apl) interpreter
on an emulated [COR24](https://github.com/sw-embed/sw-cor24-emulator)
CPU via WebAssembly.

**Live demo:** <https://sw-embed.github.io/web-sw-cor24-apl/>

Part of the [Software Wrighter COR24 Tools Project](https://sw-embed.github.io/web-sw-cor24-demos/#/).

## Features

- **REPL panel** -- printer-style terminal with 6-space APL indent,
  scrolling output, and keyboard input
- **APL glyph prettification** -- three display modes:
  *representation* (`rho`), *shorthand* (APL glyphs), and
  *literate* (`shape-of` / `reshape`)
- **Hardware panel** -- emulated COR24 S2 switch and D2 LED with
  quad-variable access (`qled`, `qsw`)
- **Demo programs** -- dropdown with pre-built examples (scalars,
  vectors, matrices, hardware I/O, shared variables, etc.)
- **File upload** -- load `.apl` source files from disk
- **Reset** -- clear workspace and restart the interpreter
- **Fully client-side** -- no server required; works offline once loaded

## APL Syntax

COR24 APL uses ASCII surface syntax with lowercase reserved words
and uppercase user identifiers:

```
      iota 5
0 1 2 3 4
      A <- 2 3 rho iota 6
      A
0 1 2
3 4 5
      rho A
2 3
      +/ iota 10
45
```

Keywords: `rho`, `iota`, `take`, `drop`, `qled`, `qsw`, `qsvo`

## Build

Requires [Trunk](https://trunkrs.dev/) and a Rust toolchain with
`wasm32-unknown-unknown`.

```bash
trunk build                # dev build to dist/
./scripts/serve.sh         # dev server on port 9957
./scripts/build-pages.sh   # release build to pages/ for GitHub Pages
```

## Technology

| Component       | Choice                              |
|-----------------|-------------------------------------|
| Language        | Rust (edition 2024)                 |
| UI framework    | Yew 0.21 (CSR)                     |
| Build tool      | Trunk                               |
| WASM bindings   | wasm-bindgen + web-sys             |
| Theme           | Catppuccin Mocha                    |
| Emulator        | cor24-emulator (path dependency)    |
| APL interpreter | sw-cor24-apl (C -> COR24 asm, embedded at build time) |

## Project Structure

```
src/
  lib.rs          App component, module declarations
  main.rs         Entry point (Yew renderer)
  repl.rs         REPL component (input/output, APL evaluation)
  control_bar.rs  Control bar (demo dropdown, file upload, reset)
  hardware.rs     Hardware panel (S2 switch, D2 LED)
  prettify.rs     APL glyph prettification engine
  demos.rs        Demo program registry
build.rs          Pre-assembles APL interpreter at build time
index.html        Trunk entry point with Catppuccin Mocha theme
scripts/
  serve.sh        Development server
  build-pages.sh  GitHub Pages release build
pages/            GitHub Pages deployment output
docs/             Design documents (architecture, PRD, plan)
```

## Links

- Blog: [Software Wrighter Lab](https://software-wrighter-lab.github.io/)
- Discord: [Join the community](https://discord.com/invite/Ctzk5uHggZ)
- YouTube: [Software Wrighter](https://www.youtube.com/@SoftwareWrighter)

## Copyright

Copyright (c) 2026 Michael A. Wright

## License

MIT
