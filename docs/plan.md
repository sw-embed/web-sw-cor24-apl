# web-sw-cor24-apl -- Implementation Plan

## Overview

Phased plan for building the browser-based APL environment. Each phase
maps to 1-2 agentrail saga steps. Follows the same patterns established
in web-sw-cor24-pcode.

## Phase 1: Scaffold Yew Project (1 step)

**Step: rust-project-init**

- Configure Cargo.toml: edition 2024, Yew 0.21 (csr), wasm-bindgen,
  web-sys, gloo, console_error_panic_hook, cor24-emulator path dep
- Set crate-type = ["cdylib", "rlib"]
- Create index.html with Catppuccin Mocha theme, Trunk directives,
  header ("APL" / "COR24 Environment"), footer with build metadata,
  GitHub corner link to sw-embed/web-sw-cor24-apl
- Create src/lib.rs with App function_component
- Create src/main.rs with Yew renderer + panic hook
- Create scripts/serve.sh (port 9957)
- Create scripts/build-pages.sh (--public-url /web-sw-cor24-apl/)
- Create build.rs with git SHA, hostname, timestamp metadata
- Verify `trunk build` compiles and serves a hello-world page

## Phase 2: REPL Panel (1-2 steps)

**Step: yew-component (REPL shell)**

- Create ReplPanel component with:
  - Scrolling output area (div with overflow-y: auto)
  - Input line at bottom with 6-space indent
  - Keyboard capture (onkeydown) with backspace support
  - Output buffer (Vec<String>) rendered as lines
  - Printer-style behavior: new output appends, panel auto-scrolls
- Style with Catppuccin Mocha colors (base background, text foreground,
  blue for prompt, green for output)
- Initially: echo input back as output (no emulator yet)

**Step: yew-component (control bar)**

- Demo dropdown (empty options initially, wired in Phase 4)
- File upload button (wired in Phase 4)
- Reset button (clears output buffer initially)

## Phase 3: Emulator Integration (1-2 steps)

**Step: integration (emulator bridge)**

- Create EmulatorBridge: instantiate EmulatorCore, load APL interpreter
  binary into memory
- Build.rs: pre-assemble APL interpreter (from sw-cor24-apl C source
  via tc24r -> cor24-assembler) at build time, embed as static bytes
- UART I/O bridge: input ring buffer (keystrokes -> UART 0xFF0100),
  output ring buffer (UART output -> display)
- Batch execution loop (gloo::timers::Timeout, ~50K instructions/tick)
- Wire ReplPanel input to UART TX, UART RX to ReplPanel output

**Step: integration (verify APL REPL)**

- Verify: type expression, see result
- Test: `iota 10`, `2 3 rho iota 6`, `+/ iota 100`
- Handle emulator startup sequence (APL interpreter init, prompt)

## Phase 4: Hardware Panel + Demos (1-2 steps)

**Step: yew-component (hardware panel)**

- Floating panel with:
  - S2 switch (initially off, click to toggle)
  - D2 LED (initially off, reflects emulator LED register 0xFF0000)
  - UART TX field (shows last sent byte, future use)
  - UART RX field (shows last received byte, future use)
- CSS: floating/positioned panel, semi-transparent background,
  draggable (stretch goal)

**Step: feature (demo registry + file upload)**

- Demo registry: pre-assembled .apl programs embedded via build.rs
- Demo dropdown populated with demo names
- Selecting demo feeds program to interpreter via UART
- File upload: read .apl file, feed contents to interpreter
- Reset button: reset emulator, reload APL interpreter binary

## Phase 5: Polish (1 step)

**Step: css-styling (polish and responsive)**

- Refine REPL panel styling (line spacing, cursor, scroll behavior)
- Hardware panel positioning and appearance
- Responsive layout considerations
- Final theme tuning
- Build pages/ for initial GitHub Pages deployment

## Phase 6: Display Modes (complete)

**Step: feature (APL glyph prettification)**

- Display mode toggle (representation / shorthand / literate)
- Shorthand mode: replace rho with APL glyph, iota with APL glyph, etc.
- Literate mode: context-sensitive display (shape-of vs reshape,
  monadic vs dyadic detection)
- Keyword entries with glyph, literate_monadic, and literate_dyadic forms
- Keyword highlighting with accent color in non-repr modes

## Dependencies

```
Phase 1 (scaffold)
  |
  v
Phase 2 (REPL panel) --- independent of emulator
  |
  v
Phase 3 (emulator integration) --- requires APL interpreter binary
  |
  v
Phase 4 (hardware panel + demos)
  |
  v
Phase 5 (polish)
  |
  v
Phase 6 (display modes) --- complete
```

## Build-time Dependency Chain

```
sw-cor24-apl (C source)
  -> tc24r (C compiler) -> COR24 assembly
  -> cor24-assembler (in cor24-emulator) -> machine code
  -> embed in build.rs as static bytes
  -> loaded into EmulatorCore at runtime
```

Note: If the APL interpreter is not yet ready for cross-compilation,
Phase 3 can use a stub binary or a simple echo program as placeholder.
