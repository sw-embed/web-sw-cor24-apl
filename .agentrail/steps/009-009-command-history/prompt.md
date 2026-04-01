Phase 9: Add command history with up/down arrow key support to the REPL.

In src/repl.rs, implement:

1. Add a history buffer (Vec<String>) to the ReplPanel component state, storing previously entered commands.
2. Add a history_index (Option<usize>) tracking position when browsing history.
3. Handle KeyboardEvent for ArrowUp and ArrowDown:
   - ArrowUp: move back in history, replace current input buffer with historical command, update cursor position.
   - ArrowDown: move forward in history, or restore the in-progress line if at the end.
4. On Enter (command submission): push the command to the history vec, reset history_index to None.
5. Keep the history buffer unbounded for the session (no persistence needed).
6. Preserve the existing keyboard handling for printable chars, Backspace, and Enter.

This is a standard readline-style history. Do NOT add line editing (Home/End/arrow-left/right) -- just up/down for history recall.