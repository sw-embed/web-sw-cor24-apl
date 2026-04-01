Phase 8: Extend APL glyph prettification to cover keywords added in recent interpreter phases.

Add the following entries to the KEYWORDS table in src/prettify.rs:

1. rev -- glyph: U+233D (circle-stile, ⌽), literate_monadic: 'reverse', literate_dyadic: None
2. cat -- glyph: U+002C (comma, ,) for monadic ravel; literate_monadic: 'ravel', literate_dyadic: Some('catenate')
3. and -- glyph: U+2227 (logical-and, ∧), literate_monadic: 'and', literate_dyadic: None (always dyadic but same name)
4. or -- glyph: U+2228 (logical-or, ∨), literate_monadic: 'or', literate_dyadic: None
5. not -- glyph: U+223C (tilde, ∼), literate_monadic: 'complement', literate_dyadic: None
6. goto -- glyph: U+2192 (right-arrow, →), literate_monadic: 'branch', literate_dyadic: None

Verify the prettification works by checking that the new demos from Phase 7 render correctly in all three display modes. Run clippy and fmt.