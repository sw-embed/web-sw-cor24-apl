Create the REPL panel component. Follow the patterns from web-sw-cor24-pcode's debugger component.

1. Create src/repl.rs with ReplPanel function_component:
   - Scrolling output area (div with overflow-y: auto, ref for auto-scroll)
   - Output buffer as Vec<String> rendered as lines
   - Input line at bottom with 6-space indent prompt
   - Keyboard capture (onkeydown on the container div) handling:
     - Enter: take current input, add prompt+input to output, echo input back (no emulator yet), clear input
     - Backspace: remove last char from input buffer
     - Printable chars: append to input buffer
   - Auto-scroll to bottom on new output
   - Printer-style behavior: new output appends at bottom

2. Update src/lib.rs:
   - Add mod repl
   - Replace placeholder with <ReplPanel />

3. Create/update src/app.css with REPL panel styles:
   - .repl-panel: flex container, full height
   - .repl-output: scrolling area, Catppuccin base background, monospace text
   - .repl-input-line: fixed at bottom, 6-space indent, blue prompt color
   - .repl-cursor: blinking cursor indicator
   - Use Catppuccin colors: base bg, text fg, blue for prompt, green for output

4. Verify trunk build compiles and the REPL panel renders with echo-back behavior.