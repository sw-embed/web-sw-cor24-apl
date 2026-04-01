# web-sw-cor24-apl -- Architecture

## Overview

Browser-based APL environment running the sw-cor24-apl interpreter
inside the COR24 emulator via WASM. The APL interpreter (written in C,
compiled to COR24 assembly via tc24r) runs on an emulated COR24 CPU.
User interaction flows through bridged UART I/O between the browser
and the emulated machine.

## System Context

```
Browser (Yew WASM app)
  |
  +-- REPL Panel (input/output, APL prettification)
  |     |
  |     +-- UART TX bridge (user keystrokes -> emulator)
  |     +-- UART RX bridge (emulator output -> display)
  |
  +-- Hardware Panel (floating)
  |     +-- S2 switch (toggle)
  |     +-- D2 LED (status indicator)
  |     +-- UART TX/RX fields (future use)
  |
  +-- Control Bar
        +-- Demo dropdown (load pre-built programs)
        +-- File upload (load .apl files)
        +-- Reset button (clear workspace)
```

## Component Diagram

```
+----------------------------------------------------------+
|  Yew App (CSR)                                           |
|                                                          |
|  +------------------+  +-----------------------------+   |
|  | HardwarePanel    |  | ReplPanel                   |   |
|  |  S2 switch       |  |  scrolling output area      |   |
|  |  D2 LED          |  |  6-space indent prompt      |   |
|  |  UART TX/RX      |  |  keyboard input + backspace |   |
|  +------------------+  +-----------------------------+   |
|                                                          |
|  +------------------+  +-----------------------------+   |
|  | ControlBar       |  | EmulatorBridge              |   |
|  |  Demo dropdown   |  |  EmulatorCore instance      |   |
|  |  File upload     |  |  UART I/O ring buffers      |   |
|  |  Reset button    |  |  Batch execution loop       |   |
|  +------------------+  +-----------------------------+   |
+----------------------------------------------------------+
         |                           |
         v                           v
  [Trunk/WASM build]         [cor24-emulator crate]
                                     |
                                     v
                            [APL interpreter binary]
                            (pre-assembled at build time
                             from sw-cor24-apl via tc24r)
```

## Data Flow

```
1. User types: A <- 2 3 rho iota 6

2. Browser captures keystrokes
     |
     v
3. UART TX bridge writes chars to emulator UART input buffer

4. EmulatorCore executes APL interpreter (batch: ~50K instructions/tick)
   - APL interpreter reads from UART (0xFF0100)
   - Tokenizes, parses (R-to-L), evaluates
   - Writes result to UART output

5. UART RX bridge reads emulator UART output buffer
     |
     v
6. ReplPanel displays output (with optional APL glyph prettification):
   0 1 2
   3 4 5

7. Prompt re-appears with 6-space indent, awaiting next input
```

## WASM Build Pipeline

```
Build time (build.rs):
  sw-cor24-apl (C source)
    -> tc24r (C compiler) -> COR24 assembly
    -> cor24-assembler -> machine code binary
    -> embedded in WASM as static bytes

Runtime:
  Trunk -> wasm-bindgen -> wasm-opt -> dist/
  deploy: rsync dist/ -> pages/
  serve: GitHub Pages at sw-embed.github.io/web-sw-cor24-apl/
```

## Path Dependency Structure

```
web-sw-cor24-apl/
  Cargo.toml
    cor24-emulator = { path = "../sw-cor24-emulator" }
    cor24-isa      = { path = "../sw-cor24-emulator/isa" }
    yew, wasm-bindgen, web-sys, gloo, console_error_panic_hook

  build.rs (build-dependencies)
    cor24-emulator = { path = "../sw-cor24-emulator" }
    (assembles APL interpreter binary at build time)
```

## APL Prettification Layer

The browser displays APL output with optional glyph substitution.
Source is always stored/transmitted as canonical ASCII.

```
Representation (stored/transmitted):  rho
Shorthand (display option):          U+2374 (APL rho glyph)
Literate (context-aware):            shape-of / reshape
```

Three display modes, toggled via buttons in the control bar:

- **Repr** -- canonical ASCII keywords (`rho`, `iota`, `take`, `drop`)
- **Glyph** -- APL symbols (rho->U+2374, iota->U+2373, take->U+2191,
  drop->U+2193, qled->U+2395 LED, qsw->U+2395 SW, qsvo->U+2395 SVO)
- **Literate** -- context-sensitive English; monadic `rho` -> "shape-of",
  dyadic `A rho B` -> "reshape" (detects dyadic usage from preceding tokens)

The prettification is a display-only transformation in the ReplPanel
component. The emulator and interpreter never see Unicode -- only
ASCII flows through UART. Keywords are highlighted with accent color
in glyph and literate modes.

## Emulator Integration

Follows the same pattern as web-sw-cor24-pcode:

- `EmulatorCore` instance created in Yew component
- APL interpreter binary loaded into emulator memory at init
- Batch execution loop (e.g., 50K instructions per `setTimeout` tick)
  prevents browser thread blocking
- UART I/O bridged via ring buffers:
  - Input: browser keystrokes -> UART data register (0xFF0100)
  - Output: UART output -> browser display
- S2 switch mapped to emulator's switch input
- D2 LED mapped to emulator's LED output (0xFF0000)

## Key Technology Choices

| Choice              | Value                                    |
|---------------------|------------------------------------------|
| Language            | Rust (edition 2024)                      |
| UI framework        | Yew 0.21 (CSR only)                     |
| Build tool          | Trunk                                    |
| WASM bindings       | wasm-bindgen + web-sys                   |
| Theme               | Catppuccin Mocha                         |
| Deployment          | GitHub Pages (pages/ directory)          |
| Dev server port     | 9957                                     |
| Emulator            | cor24-emulator (path dependency)         |
| APL interpreter     | sw-cor24-apl (C -> COR24 asm, embedded)  |
