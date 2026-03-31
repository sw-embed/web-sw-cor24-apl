# web-sw-cor24-apl -- UI/UX Design

## Layout Overview

Single-page application with a full-height REPL panel as the primary
interface. A floating hardware panel overlays the REPL. A control bar
sits between the header and the REPL panel.

```
+----------------------------------------------------------+
| Header: "APL" | "COR24 Environment"        [GitHub corner]|
+----------------------------------------------------------+
| Control Bar: [Demo v] [Upload] [Reset]                    |
+----------------------------------------------------------+
|                                                           |
|  REPL Panel (scrolling output area)                       |
|                                                           |
|        A <- 2 3 rho iota 6                                |
|  0 1 2                                                    |
|  3 4 5                                                    |
|        +/ iota 100                                        |
|  4950                                                     |
|        _  <-- cursor here, 6-space indent                 |
|                                                           |
|  +------------------+                                     |
|  | Hardware Panel   |                                     |
|  | S2: [off]        |                                     |
|  | D2: (o)          |                                     |
|  | TX: --  RX: --   |                                     |
|  +------------------+                                     |
|                                                           |
+----------------------------------------------------------+
| Footer: MIT | (c) 2026 MAW | COR24-TB | sha | host | ts |
+----------------------------------------------------------+
```

## REPL Panel Behavior

The REPL emulates a **printer terminal** (not a screen editor):

- Output scrolls upward continuously
- Input only happens at the bottom line
- No cursor movement up/down or editing previous lines
- Backspace erases the last typed character on the current line
- Enter submits the current line to the interpreter
- After submission, output appears below, then a new prompt

### Prompt

- 6 spaces of indentation (traditional APL convention)
- No prompt character -- just the indent
- Cursor blinks after the indent (or after typed characters)

### Input Handling

- Printable ASCII characters append to current input line
- Backspace: delete last character (stop at indent boundary)
- Enter: submit line to UART TX bridge
- No tab, no arrow keys (initially)
- Focus: REPL panel captures keyboard when page is focused

### Output Display

- Interpreter output appears line by line as received from UART RX
- Each output line left-aligned (interpreter handles formatting)
- Matrices: one row per line, right-justified columns (done by
  the APL interpreter, not the browser)
- Errors: displayed as-is from interpreter (e.g., "SYNTAX ERROR")

### Scrolling

- Auto-scroll to bottom on new output or input
- User can scroll up to review history
- Scroll lock: if user has scrolled up, don't auto-scroll
  (resume auto-scroll when user scrolls to bottom)

## Hardware Panel

Floating panel in the bottom-right corner of the REPL area.

### S2 Switch

- Visual toggle switch (on/off)
- Initially off
- Maps to COR24 emulator switch input
- Click to toggle

### D2 LED

- Circular indicator (lit = green, off = dark)
- Initially off
- Reflects emulator LED register (0xFF0000)
- Updated each execution tick

### UART TX/RX Fields

- Small text fields showing last byte sent/received
- For future diagnostic use
- Initially display "--" or "00"

### Panel Styling

- Semi-transparent dark background (--mantle with opacity)
- Compact layout, small font
- Positioned bottom-right, above footer
- Does not block REPL input line
- Future: draggable

## Control Bar

Horizontal bar below the header.

### Demo Dropdown

- HTML select element
- First option: "(select demo)" disabled placeholder
- Options populated from demo registry (build-time embedded)
- Selecting a demo: feeds program to interpreter via UART
- After execution, interpreter returns to REPL prompt

### File Upload Button

- Hidden file input + styled button ("Upload .apl")
- Accepts .apl and .txt files
- On upload: read file contents, feed to interpreter via UART
- Large files: feed line by line with pacing (avoid UART overflow)

### Reset Button

- Styled button ("Reset")
- Resets emulator (reload APL interpreter binary)
- Clears REPL output buffer
- Returns to fresh workspace with 6-space indent prompt

## Catppuccin Mocha Theme

All colors from the Catppuccin Mocha palette:

| Element            | Color variable | Hex       |
|--------------------|---------------|-----------|
| Page background    | --base        | #1e1e2e   |
| Panel background   | --mantle      | #181825   |
| Surface borders    | --surface1    | #45475a   |
| Primary text       | --text        | #cdd6f4   |
| Header title       | --blue        | #89b4fa   |
| Prompt indent      | --text        | #cdd6f4   |
| User input text    | --text        | #cdd6f4   |
| Output text        | --green       | #a6e3a1   |
| Error text         | --red         | #f38ba8   |
| LED on             | --green       | #a6e3a1   |
| LED off            | --surface0    | #313244   |
| Switch on          | --blue        | #89b4fa   |
| Switch off         | --surface1    | #45475a   |
| Button             | --surface0    | #313244   |
| Button hover       | --surface1    | #45475a   |
| Footer text        | --overlay0    | #6c7086   |
| Links              | --blue        | #89b4fa   |

Font: JetBrains Mono / Fira Code / Cascadia Code / monospace fallback.
Font size: 15px base, 0.85em footer.

## Component Hierarchy (Yew)

```
App
  +-- Header (static HTML in App)
  +-- ControlBar
  |     +-- DemoDropdown (select element)
  |     +-- FileUpload (input[type=file] + button)
  |     +-- ResetButton
  +-- ReplPanel
  |     +-- OutputArea (scrolling div)
  |     +-- InputLine (captures keyboard)
  +-- HardwarePanel (floating)
  |     +-- S2Switch
  |     +-- D2Led
  |     +-- UartDisplay
  +-- Footer (static HTML in App)
```

State lives primarily in the top-level App or a shared context:
- `output_lines: Vec<String>` -- REPL output history
- `input_buffer: String` -- current input line
- `emulator: EmulatorCore` -- the running emulator instance
- `demos: Vec<Demo>` -- embedded demo programs

## APL Glyph Rendering (Future)

Three display modes, toggled by a control:

### Representation Mode (v1 default)
Display exactly what the interpreter outputs: `rho`, `iota`, etc.
No transformation.

### Shorthand Mode (future)
Post-process output to replace ASCII tokens with Unicode glyphs:
- `rho` -> `⍴`
- `iota` -> `⍳`
- `take` -> `↑`
- `drop` -> `↓`
- `rev` -> `⌽`
- `cat` -> `,`

Applied as a display filter on UART RX output before rendering.

### Literate Mode (future)
Context-sensitive replacement using arity detection:
- monadic `rho X` -> `shape-of X`
- dyadic `A rho B` -> `A reshape B`

Requires lightweight parsing of the display output. More complex,
deferred to Phase 6.

## Responsive Considerations

- REPL panel fills available height (flex: 1)
- Hardware panel repositions or collapses on narrow viewports
- Minimum viable width: ~400px (phone landscape)
- No horizontal scroll in REPL (long lines wrap or truncate)
- Control bar wraps on narrow screens
