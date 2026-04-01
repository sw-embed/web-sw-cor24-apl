# web-sw-cor24-apl -- Product Requirements Document

## Purpose

Provide a browser-based APL environment that runs the sw-cor24-apl
interpreter on an emulated COR24 CPU via WASM. The interface emulates
a printer-style terminal: output scrolls upward, input happens at the
bottom with a 6-space APL indent, and the user interacts through a
single REPL panel.

## Target Users

- The developer (demonstrating APL on COR24 in the browser)
- Anyone exploring APL concepts with a minimal, educational interpreter
- Visitors to the GitHub Pages demo site

## Core Features

### F-1: Single-Panel REPL

- Large scrolling output area (printer-style, not a full editor)
- Input at the bottom only, with 6-space indent prompt
- Keyboard input with backspace support
- Output scrolls up as new results appear
- Monospace font (Catppuccin Mocha theme)

### F-2: Floating Hardware Panel

- Emulated COR24 S2 switch (initially off, toggleable)
- Emulated D2 LED (initially off, reflects emulator state)
- UART TX and RX fields (placeholder for future use)
- Floating/draggable panel that doesn't obstruct REPL

### F-3: Demo Dropdown

- Dropdown selector with pre-built APL demo programs
- Selecting a demo loads it into the interpreter
- Demos pre-assembled at build time (embedded in WASM)

### F-4: File Upload

- Upload an APL source file (.apl) from local filesystem
- File contents fed to the interpreter via UART bridge
- Supports loading external programs into the workspace

### F-5: Reset Button

- Clears the workspace (equivalent to `)CLEAR`)
- Resets emulator state
- Returns to empty workspace with 6-space indent prompt

### F-6: APL Glyph Display (three modes)

- Toggle between three display modes:
  - **Representation**: canonical ASCII (`rho`, `iota`, etc.)
  - **Shorthand**: APL glyphs (rho, iota, take, drop, quad-LED, quad-SW)
  - **Literate**: context-sensitive English names (`shape-of` / `reshape`,
    `index-gen`, `take`, `drop`); monadic vs dyadic forms detected
    from context

### F-7: Emulator Execution

- APL interpreter runs inside cor24-emulator in WASM
- Batch execution loop prevents browser blocking
- UART I/O bridge connects browser input to emulator

## Non-Functional Requirements

### NFR-1: Performance

- Emulator batch execution must not block the browser UI
- Target: responsive input with < 100ms perceived latency
  for simple expressions

### NFR-2: Browser Compatibility

- Must work in modern browsers (Chrome, Firefox, Safari, Edge)
- WASM required (no fallback)
- No server backend -- fully client-side

### NFR-3: Offline Capable

- Once loaded, the app works without network access
- All demos and interpreter code embedded in the WASM bundle

### NFR-4: Deployment

- GitHub Pages from pages/ directory
- URL: https://sw-embed.github.io/web-sw-cor24-apl/
- .nojekyll file for proper static serving

## In Scope (v1)

- Single REPL panel with printer-style scrolling output
- Floating hardware panel (S2 switch, D2 LED, UART fields)
- Demo dropdown with pre-built examples
- File upload for .apl source files
- Reset button
- Canonical ASCII display mode
- Catppuccin Mocha theme
- GitHub corner link
- Footer with build metadata

## Out of Scope (v1)

- Multi-panel editor / IDE
- Syntax highlighting in the input area
- APL glyph keyboard input (type Unicode directly)
- APL glyph keyboard input (type Unicode directly)
- Variable inspector / workspace browser
- Breakpoint debugging
- Save/export workspace
- Mobile-optimized layout
- Server-side execution

## Success Criteria

1. App loads in browser and shows REPL with 6-space indent prompt
2. User can type `iota 10` and see `0 1 2 3 4 5 6 7 8 9`
3. User can type `A <- 2 3 rho iota 6` then `A` and see a 2x3 matrix
4. Demo dropdown loads and executes a pre-built program
5. Reset button clears workspace and returns to empty prompt
6. Hardware panel shows S2 switch and D2 LED
7. Runs entirely client-side with no server dependencies
