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
        name: "Ceil & Floor",
        description: "Max (ceil) and min (floor) of integers",
        source: "\
3 ceil 5
3 floor 5
1 2 3 ceil 3 2 1
1 2 3 floor 3 2 1
ceil/ 4 1 7 2
floor/ 4 1 7 2
",
    },
    Demo {
        name: "Character Literals",
        description: "String values, assignment, and shape",
        source: "\
'hello'
'A'
''
A <- 'test'
A
rho 'hello'
rho ''
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
        name: "Compress",
        description: "Boolean mask filtering of arrays",
        source: "\
1 0 1 compress 10 20 30
1 1 1 compress 4 5 6
0 0 0 compress 4 5 6
(3 = iota 5) compress iota 5
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
        name: "Horse Race",
        description: "4 named horses race with track visualization",
        source: "\
qrl <- 42
NH <- 4
GOAL <- 15
POS <- NH rho 0
RND <- 0
NAMES <- 'Thndr' 'Lghtn' 'Storm' 'Blaze'
#
del R <- TRACK X
R <- 0
I <- 0
SHOW: R <- (I pick NAMES) cat '|' cat (X[I] rho '#')
qout <- R
I <- I + 1
goto (I < NH)/SHOW
del
#
del R <- RACE X
R <- 0
NEXT: RND <- RND + 1
qout <- '=== Round ' cat fmt RND
POS <- POS + roll NH rho 3
TRACK POS
LEAD <- ceil/ POS
qout <- 'Leader at ' cat fmt LEAD
DONE <- or/ POS >= GOAL
goto (DONE = 0)/NEXT
WIN <- (POS >= GOAL) compress iota NH
NW <- rho WIN
qout <- 'Race over!'
goto (NW > 1)/TIE
qout <- 'Winner: ' cat ((0 pick WIN) pick NAMES)
goto 0
TIE: qout <- (fmt NW) cat '-way tie!'
del
#
qout <- '*** HORSE RACE ***'
Z <- RACE 0
)OFF
",
    },
    Demo {
        name: "Horse Race (Idiomatic)",
        description: "Compact APL horse race with names and track",
        source: "\
qrl <- 42
NAMES <- 'Lucky' 'Thndr' 'Shadw' 'Comet' 'Blaze'
NH <- 5
#
del R <- SHOW X
R <- 0
I <- 0
S: qout <- (I pick NAMES) cat '|' cat ((I pick POS) rho '#')
I <- I + 1
goto (I < NH)/S
del
#
del R <- RACE X
R <- 0
POS <- NH rho 0
qout <- 'THE RACE IS ON!'
L: POS <- POS + roll NH rho 3
SHOW 0
qout <- ''
goto (0 = or/ POS >= 15)/L
qout <- 'WINNER: ' cat ((0 pick (POS = ceil/ POS) compress iota NH) pick NAMES)
del
#
Z <- RACE 0
)OFF
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
        name: "String Operations",
        description: "String repetition, cycling, and catenation",
        source: "\
5 rho '#'
3 rho 'ab'
rho 5 rho '#'
'hi' cat ' world'
'a' cat 'b'
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
        name: "User-Defined Functions",
        description: "Define and call functions with del",
        source: "\
del R <- DOUBLE X
R <- X + X
del
DOUBLE 7
DOUBLE iota 4
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
