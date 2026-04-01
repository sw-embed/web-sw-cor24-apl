Phase 7: Add new demo programs showcasing interpreter features not yet demoed in the web UI.

Add the following demos to src/demos.rs, inserting them in logical order among existing demos:

1. **Reverse & Catenate** -- 'rev 1 2 3 4 5', 'cat 2 3 rho iota 6' (ravel), '1 2 3 cat 4 5 6' (catenate)
2. **Take & Drop** -- 'A <- iota 10', '3 take A', '_3 take A' (negative = from end), '2 drop A', '_2 drop A', '2 take 2 3 rho iota 6' (matrix take)
3. **Bracket Indexing** -- 'V <- 10 20 30 40 50', 'V[0]', 'V[3]', 'V[2] <- 99', 'V'
4. **Bitwise Operations** -- '7 and 3', '5 or 3', 'not 0', '1 2 3 and 3 2 1', practical example: 'MMIO qsvo 242', 'MMIO[257] and 2'
5. **Error Handling** -- showcase all 6 error types: SYNTAX ERROR (@bad), DOMAIN ERROR (1/0), VALUE ERROR (UNDEF), LENGTH ERROR (1 2 3+4 5), RANK ERROR (+/2 3 rho iota 6), then show REPL recovers
6. **Control Flow** -- a counting loop: 'I <- 5', 'LOOP:', '[] <- I', 'I <- I - 1', 'goto (I)/LOOP', '[] <- 0' to show goto/label/conditional-branch

Ensure all demos compile and run correctly on the interpreter. Keep the Demo struct (name, description, source) pattern. Put Hardware I/O and Shared Variables last since they are hardware-specific.