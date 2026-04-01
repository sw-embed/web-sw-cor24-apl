Phase 13: Show index origin (⎕IO) indicator in the UI.

The COR24 APL interpreter uses 0-based indexing (⎕IO←0). Standard APL defaults to ⎕IO←1, so users need a visible indicator of the current index origin.

1. Add a ⎕IO indicator to the control bar or status area showing the current index origin (0 or 1).
2. The indicator should be always visible so users understand why iota 5 produces 0 1 2 3 4 rather than 1 2 3 4 5.
3. For now this is read-only display. Future work: add ⎕IO support for getting/setting index origin via the interpreter.