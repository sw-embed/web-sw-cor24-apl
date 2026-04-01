//! Demo registry — APL programs fed as text via UART to the interpreter.

pub struct Demo {
    pub name: &'static str,
    pub description: &'static str,
    pub source: &'static str,
}

/// Demo programs in COR24 APL ASCII syntax (alphabetical order).
/// Each line is sent to the interpreter via UART as if the user typed it.
pub const DEMOS: &[Demo] = &[
    Demo {
        name: "Bitwise Operations",
        description: "Bitwise and, or, not on integers",
        source: "\
7 and 3
5 or 3
not 0
not 1
1 2 3 and 3 2 1
'MMIO' qsvo 242
MMIO[257] and 2
",
    },
    Demo {
        name: "Bracket Indexing",
        description: "Read and write vector elements by index",
        source: "\
V <- 10 20 30 40 50
V[0]
V[3]
V[2] <- 99
V
",
    },
    Demo {
        name: "Comparison Operators",
        description: "Equality, relational, and not-equal tests",
        source: "\
3 = 3
3 = 4
5 > 3
3 > 5
5 < 3
3 < 5
5 >= 5
5 >= 3
3 >= 5
5 <= 5
3 <= 5
5 <= 3
3 != 3
3 != 4
1 2 3 >= 2
1 2 3 = 1 2 3
1 2 3 = 1 0 3
",
    },
    Demo {
        name: "Control Flow",
        description: "Counting loop with goto and labels",
        source: "\
I <- 5
LOOP:
[] <- I
I <- I - 1
goto (I)/LOOP
[] <- 0
",
    },
    Demo {
        name: "Edge Cases",
        description: "Empty vectors, single-element, nested parens",
        source: "\
iota 0
rho iota 0
rev iota 0
(iota 0) cat iota 3
(iota 0) + (iota 0)
0 take iota 5
5 drop iota 5
0 rho 1 2 3
rho 0 rho 1 2 3
+/ iota 0
iota 1
+/ iota 1
1 rho 5
rho 1 rho 5
1 1 rho 5
rho 1 1 rho 5
((((1 + 2))))
(((((((42)))))))
(1 + (2 + (3 + (4 + 5))))
",
    },
    Demo {
        name: "Error Handling",
        description: "All six error types, REPL recovers",
        source: "\
@bad
1 / 0
UNDEF
1 2 3 + 4 5
+/ 2 3 rho iota 6
3 + 4
",
    },
    Demo {
        name: "Hardware I/O",
        description: "LED and switch via quad-variables",
        source: "\
qled
qled <- 1
qled
qled <- 0
qled
qsw
qled <- qsw
qled
",
    },
    Demo {
        name: "Iota & Reduce",
        description: "Index generation and reduction",
        source: "\
iota 5
iota 10
+/ 1 2 3 4 5
+/ iota 10
-/ 1 2 3
*/ 1 2 3 4
",
    },
    Demo {
        name: "Matrix Take & Drop",
        description: "Take and drop rows/columns from matrices",
        source: "\
1 3 take 3 3 rho iota 9
_1 3 take 3 3 rho iota 9
1 3 drop 3 3 rho iota 9
_1 3 drop 3 3 rho iota 9
2 3 take 3 3 rho iota 9
rho 2 3 take 3 3 rho iota 9
",
    },
    Demo {
        name: "Matrices",
        description: "Matrix operations with reshape",
        source: "\
2 3 rho iota 6
(2 3 rho iota 6) + (2 3 rho 10 20 30 40 50 60)
10 + 2 3 rho iota 6
(2 3 rho iota 6) * 2
",
    },
    Demo {
        name: "Multiline Programs",
        description: "Line entry, list, run, and erase",
        source: "\
[1] N <- 0
[2] LOOP: N <- N + 1
[3] [] <- N
[4] goto (5 - N)/LOOP
[5] [] <- 99
)LIST
)RUN
)ERASE
)LIST
",
    },
    Demo {
        name: "Reshape",
        description: "Rho for reshape and shape-of",
        source: "\
3 rho 1 2 3
5 rho 1 2 3
2 3 rho 1 2 3 4 5 6
rho 2 3 rho iota 6
",
    },
    Demo {
        name: "Reverse & Catenate",
        description: "Reverse, ravel, and catenate arrays",
        source: "\
rev 1 2 3 4 5
cat 2 3 rho iota 6
1 2 3 cat 4 5 6
",
    },
    Demo {
        name: "Scalars",
        description: "Scalar arithmetic and parentheses",
        source: "\
3 + 4
10 - 3
6 * 4
20 / 5
2 + 3 * 4
(2 + 3) * 4
_3 + 10
",
    },
    Demo {
        name: "Shared Variables",
        description: "MMIO via qsvo 242 (AP 242 shared variable coupling)",
        source: "\
'MMIO' qsvo 242
MMIO[0]
MMIO[0] <- 1
MMIO[0]
MMIO[257]
MMIO[0] <- 0
MMIO[0]
",
    },
    Demo {
        name: "System Commands",
        description: "Workspace management with )VARS and )CLEAR",
        source: "\
A <- 5
B <- 10
)VARS
A + B
)CLEAR
)VARS
",
    },
    Demo {
        name: "Take & Drop",
        description: "Select elements from the front or end",
        source: "\
A <- iota 10
3 take A
_3 take A
2 drop A
_2 drop A
2 take 2 3 rho iota 6
",
    },
    Demo {
        name: "Variables",
        description: "Assignment and variable use",
        source: "\
A <- 5
A + 3
B <- A * 2
B
A + B
)VARS
",
    },
    Demo {
        name: "Vectors",
        description: "Vector literals and element-wise operations",
        source: "\
1 2 3
1 2 3 + 10 20 30
2 * 1 2 3
1 2 3 + 10
rho 1 2 3 4 5
",
    },
];
