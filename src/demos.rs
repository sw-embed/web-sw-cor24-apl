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
V assign 10 20 30 40 50
V[0]
V[3]
V[2] assign 99
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
A assign 'test'
A
rho 'hello'
rho ''
",
    },
    Demo {
        name: "Comments",
        description: "Full-line and inline comments",
        source: "\
comment This is a full-line comment
A assign 5 comment inline comment after expression
qout assign A + 3
comment Another comment
B assign 10 20 30
qout assign +/ B comment sum of vector
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
I assign 5
LOOP:
[] assign I
I assign I - 1
goto (I)/LOOP
[] assign 0
",
    },
    Demo {
        name: "Delay",
        description: "Pause execution with qdl (milliseconds)",
        source: "\
qout assign 'start'
qdl 100
qout assign 'after 100ms'
qdl 50
qout assign 'after 50ms'
N assign 200
qdl N
qout assign 'after 200ms'
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
        name: "Format",
        description: "Convert numbers to strings with fmt",
        source: "\
fmt 42
fmt _7
fmt 0
fmt 1 2 3
rho fmt 42
rho fmt 100
'ROUND ' cat fmt 5
'Score: ' cat fmt 123
",
    },
    Demo {
        name: "Hardware I/O",
        description: "LED and switch via quad-variables",
        source: "\
qled
qled assign 1
qled
qled assign 0
qled
qsw
qled assign qsw
qled
",
    },
    Demo {
        name: "Horse Race",
        description: "4 named horses race with track visualization",
        source: "\
comment Horse Race -- Full Version
comment 4 named horses race with track visualization
qio assign 0
qrl assign 42
NH assign 4
GOAL assign 15
POS assign NH rho 0
RND assign 0
NAMES assign 'Thndr' 'Lghtn' 'Storm' 'Blaze'
comment
del R assign TRACK X
R assign 0
I assign 0
SHOW: R assign (I pick NAMES) cat '|' cat (X[I] rho '#')
qout assign R
I assign I + 1
goto (I < NH)/SHOW
del
comment
del R assign RACE X
R assign 0
NEXT: RND assign RND + 1
qout assign '=== Round ' cat fmt RND
POS assign POS + roll NH rho 3
TRACK POS
LEAD assign ceil/ POS
qout assign 'Leader at ' cat fmt LEAD
DONE assign or/ POS >= GOAL
goto (DONE = 0)/NEXT
WIN assign (POS >= GOAL) compress iota NH
NW assign rho WIN
qout assign 'Race over!'
goto (NW > 1)/TIE
qout assign 'Winner: ' cat ((0 pick WIN) pick NAMES)
goto 0
TIE: qout assign (fmt NW) cat '-way tie!'
del
comment
qout assign '*** HORSE RACE ***'
Z assign RACE 0
)OFF
",
    },
    Demo {
        name: "Horse Race (Idiomatic)",
        description: "Compact APL horse race with names and track",
        source: "\
qio assign 0
qrl assign 42
NAMES assign 'Lucky' 'Thndr' 'Shadw' 'Comet' 'Blaze'
NH assign 5
comment
del R assign SHOW X
R assign 0
I assign 0
S: qout assign (I pick NAMES) cat '|' cat ((I pick POS) rho '#')
I assign I + 1
goto (I < NH)/S
del
comment
del R assign RACE X
R assign 0
POS assign NH rho 0
qout assign 'THE RACE IS ON!'
L: POS assign POS + roll NH rho 3
SHOW 0
qout assign ''
goto (0 = or/ POS >= 15)/L
qout assign 'WINNER: ' cat ((0 pick (POS = ceil/ POS) compress iota NH) pick NAMES)
del
comment
Z assign RACE 0
)OFF
",
    },
    Demo {
        name: "Index Origin",
        description: "Switch between 1-based and 0-based indexing with qio",
        source: "\
qio
iota 5
qio assign 0
iota 5
V assign 10 20 30
V[0]
qio assign 1
iota 5
V[1]
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
        name: "Lazy Iota",
        description: "Large iota without WS FULL (lazy evaluation)",
        source: "\
comment Lazy iota: iota N uses only 4 heap words regardless of N
rho iota 999999
5 take iota 999999
5 take 5 drop iota 999999
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
[1] N assign 0
[2] LOOP: N assign N + 1
[3] [] assign N
[4] goto (5 - N)/LOOP
[5] [] assign 99
)LIST
)RUN
)ERASE
)LIST
",
    },
    Demo {
        name: "Nested Arrays",
        description: "String arrays and nested structure",
        source: "\
'cat' 'dog' 'fish'
rho 'cat' 'dog' 'fish'
A assign 'hello' 'world'
A
rho A
'one' 'two' 'three' 'four'
",
    },
    Demo {
        name: "OR & AND Reduce",
        description: "Logical or/ and and/ reduction",
        source: "\
comment or/ reduce tests
or/ 0 0 1 0
or/ 0 0 0 0
or/ 1 1 1 1
comment and/ reduce tests
and/ 1 1 1 1
and/ 1 1 0 1
and/ 0 0 0 0
comment mixed with expressions
A assign 0 1 0 1
or/ A
and/ A
",
    },
    Demo {
        name: "Quad Output",
        description: "Explicit printing with qout in functions",
        source: "\
qout assign 42
qout assign 1 2 3
qout assign 'hello'
A assign 10
qout assign A + 5
del R assign SHOW X
R assign X
qout assign 'value:'
qout assign X
del
SHOW 7
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
MMIO[0] assign 1
MMIO[0]
MMIO[257]
MMIO[0] assign 0
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
A assign 5
B assign 10
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
A assign iota 10
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
del R assign DOUBLE X
R assign X + X
del
DOUBLE 7
DOUBLE iota 4
",
    },
    Demo {
        name: "Variables",
        description: "Assignment and variable use",
        source: "\
A assign 5
A + 3
B assign A * 2
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
