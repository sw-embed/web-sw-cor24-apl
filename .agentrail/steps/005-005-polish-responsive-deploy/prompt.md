Phase 5: Polish, responsive design, and GitHub Pages deployment.

1. Responsive design updates in src/app.css and index.html:
   - Hardware panel: reposition or collapse on viewports < 600px (move above REPL or hide behind toggle)
   - Control bar: flex-wrap so buttons wrap on narrow screens  
   - REPL panel: ensure no horizontal scroll, long lines wrap
   - Minimum viable width ~400px (phone landscape)

2. Polish the UI:
   - Ensure cursor blink animation works in the REPL
   - Add focus outline indication for accessibility
   - Smooth scrolling behavior for REPL output
   - Ensure keyboard focus returns to REPL after control bar interactions

3. Build and deploy to GitHub Pages:
   - Run scripts/build-pages.sh to create release build in pages/
   - Verify pages/ contains index.html, .wasm, .js, .css files
   - Ensure .nojekyll file is present

4. Verify trunk build compiles with no warnings.