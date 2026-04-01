Phase 14: Change qsvo to accept string left argument like IBM 5100 APL.

In standard APL, shared variable offer syntax is: 'MMIO' ⎕SVO 242 (string argument, not bare identifier). The COR24 interpreter currently uses bare identifiers: MMIO qsvo 242.

1. Update the C interpreter (sw-cor24-apl) tokenizer/parser to accept a string literal as the left argument to qsvo: 'MMIO' qsvo 242
2. The string value becomes the variable name to create/couple.
3. Update all tests in sw-cor24-apl to use the new syntax.
4. Update demos in web-sw-cor24-apl (Shared Variables, Bitwise Operations) to use 'MMIO' qsvo 242.
5. This is a cross-repo change: sw-cor24-apl (interpreter) then web-sw-cor24-apl (demos).