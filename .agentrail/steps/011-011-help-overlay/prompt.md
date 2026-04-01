Phase 11: Add an APL quick-reference help overlay.

Create a toggleable help overlay/panel that shows APL syntax reference for this interpreter. Add a '?' or 'Help' button to the control bar that opens it.

Content should be a concise reference card covering:
- Arithmetic: + - * / (element-wise, scalar extension)
- Array primitives: iota, rho (monadic/dyadic), take, drop, rev, cat
- Reduce: +/ -/ */
- Bitwise: and, or, not
- Variables: NAME <- expr, )VARS, )CLEAR
- Bracket indexing: V[N], V[N] <- expr
- Hardware: qled, qsw, qsvo, MMIO[N]
- Control flow: goto LABEL, goto (cond)/LABEL, LABEL:
- Output: [] <- expr
- Negative literals: _N (underscore prefix)
- System commands: )OFF, )CLEAR, )VARS
- Evaluation: right-to-left, use parens to override

Style it as a modal overlay with Catppuccin Mocha colors (surface0 background, text color), dismissable by clicking outside or pressing Escape. Keep it readable and compact -- a reference card, not a tutorial. Use a <table> or definition list for the entries.