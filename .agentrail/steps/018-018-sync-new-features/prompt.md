Phase 18: Sync new CLI features to Web UI — demos and prettification.

The sw-cor24-apl interpreter has added several features since the last sync (step 015). Add corresponding web UI demos and extend prettification.

## New Demos to Add (5)

Add these to src/demos.rs in alphabetical order within the existing list:

1. **Ceil & Floor** — from samples/batch-max-min.apl (ASCII syntax: ceil, floor)
   - Dyadic: 3 ceil 5, 3 floor 5
   - Vector: 1 2 3 ceil 3 2 1
   - Reduce: ceil/ 4 1 7 2, floor/ 4 1 7 2

2. **Character Literals** — from samples/18-char-type.apl (ASCII syntax)
   - String literals: 'hello', 'A', '' (empty)
   - Assignment: A <- 'test'
   - Shape: rho 'hello'

3. **Compress** — from samples/batch-compress.apl (ASCII syntax: compress)
   - Boolean mask: 1 0 1 compress 10 20 30
   - Filtering: (3 = iota 5) compress iota 5

4. **String Operations** — from samples/19-string-ops.apl (use ASCII syntax)
   - Repetition: 5 rho '#'
   - Cycling: 3 rho 'ab'
   - Catenation: 'hi' cat ' world'

5. **User-Defined Functions** — from samples/batch-functions.apl (ASCII syntax: del)
   - Definition: del R <- DOUBLE X / R <- X + X / del
   - Calling: DOUBLE 7, DOUBLE iota 4

## Prettify Extensions (4 new keywords in src/prettify.rs)

Add to KEYWORDS array:
- compress: glyph="/", monadic=None, dyadic="compress"
- ceil: glyph="⌈", monadic="ceiling", dyadic="max"
- floor: glyph="⌊", monadic="floor", dyadic="min"
- del: glyph="∇", monadic="define-function", dyadic=None

## Also Update

- Help overlay (src/help.rs) Quick Reference tab: add rows for compress, ceil, floor, del
- Tutorial tab: no changes needed (existing lessons still valid)

## Verification

- cargo clippy, fmt, test
- Build pages with ./scripts/build-pages.sh
- Launch dev server with ./scripts/serve.sh
- Playwright: navigate, select each new demo, verify output renders
- Playwright: check glyph mode shows new glyphs for new keywords