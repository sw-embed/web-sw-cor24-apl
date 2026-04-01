Phase 16: Add a Tutorial tab to the help dialog (launched by the ? button).

Currently the help overlay has a single Quick Reference view. Restructure it into two tabs: 'Quick Reference' (existing content) and 'Tutorial'. The Tutorial tab should introduce APL concepts progressively from beginner to advanced, organized as a guided learning path. Suggested progression:

1. Scalars & Arithmetic (+ - * /, negatives with _N, right-to-left eval)
2. Variables (assignment with <-, )VARS, )CLEAR)
3. Vectors (literal vectors, element-wise ops, scalar extension)
4. Iota & Reduce (iota N, +/ */ -/ reduction)
5. Reshape & Shape (rho for shape-of and reshape, rank)
6. Take & Drop (positive/negative, on vectors and matrices)
7. Reverse & Catenate (rev, cat, ravel)
8. Bracket Indexing (V[i], V[i] <- val)
9. Matrices (2D reshape, matrix arithmetic)
10. Comparison Operators (= > < >= <= <>, boolean results)
11. Control Flow (labels, goto, conditional branch)
12. System Commands ()VARS, )CLEAR, )LIST, )RUN, )ERASE)
13. Multiline Programs ([N] line entry, )LIST, )RUN)
14. User-Defined Functions (del syntax)
15. Hardware I/O (qled, qsw quad-variables)
16. Shared Variables (qsvo, AP 242 coupling, MMIO)
17. Bitwise Operations (and, or, not)
18. Edge Cases & Error Handling (empty vectors, error recovery)

Each section should have a brief explanation and a small runnable example. Style consistently with the existing help overlay using Catppuccin Mocha theme. Tab switching should be smooth with no page reload.