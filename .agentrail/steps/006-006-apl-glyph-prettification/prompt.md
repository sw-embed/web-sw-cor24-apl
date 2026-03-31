Phase 6: APL glyph prettification display modes.

1. Create a prettification module (src/prettify.rs):
   - Map ASCII keywords to APL glyphs: rho→⍴, iota→⍳, take→↑, drop→↓, each→¨, reduce→/ (already symbol)
   - Map to literate names: rho→shape-of/reshape (context-sensitive), iota→index-generator, take→take, drop→drop
   - Three display modes: representation (ASCII source), shorthand (APL glyphs), literate (English names)
   - Context sensitivity: monadic vs dyadic detection for display labels

2. Add display mode toggle to the control bar (src/control_bar.rs):
   - Three-way toggle or dropdown: Repr | Glyph | Literate
   - Pass selected mode as prop to REPL panel

3. Apply prettification to REPL output (src/repl.rs):
   - Process each output line through the prettifier before rendering
   - Only prettify the echoed input lines (the 6-space indented lines), not interpreter output
   - Preserve the underlying ASCII representation for re-evaluation

4. Style the prettified output (src/app.css):
   - APL glyphs in a distinct color (e.g., Catppuccin blue/mauve)
   - Literate names in a softer color (e.g., Catppuccin overlay)

5. Verify trunk build compiles with no warnings and deploy to GitHub Pages.