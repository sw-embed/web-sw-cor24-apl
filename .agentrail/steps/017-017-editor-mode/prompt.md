Phase 17: Design and implement an editor mode for APL programs.

Add an editor mode that allows the user to:
1. Edit a loaded Demo program before running it
2. Edit an uploaded file before running it
3. Create a new empty file and write APL code from scratch

UI design considerations:
- Add an 'Edit' button or mode toggle accessible from the control bar
- When a demo is selected or file uploaded, instead of immediately sending to REPL, open in editor
- Editor should be a textarea or contenteditable area with monospace font, Catppuccin Mocha styling
- Provide a 'Run' button to send the editor contents to the REPL via UART
- Provide a 'Download' link/button to save the editor contents as a .apl file
- Provide a 'New' button to clear the editor for a fresh program
- Editor should support the same APL glyph prettification display as the REPL (show glyphs in chosen display mode)
- Consider whether editor is a panel/tab alongside REPL or a modal overlay
- The editor contents should persist across demo switches (warn if unsaved changes)

Build, test, and verify the editor mode works end-to-end: load demo -> edit -> run -> see output in REPL. Also test: new file -> write code -> run -> download.