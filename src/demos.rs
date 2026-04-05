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
        name: "Abs & Residue",
        description: "Absolute value and modulo operators",
        source: "\
comment abs (absolute value) and residue (modulo)
comment monadic abs
quad assign abs _5
quad assign abs 3
quad assign abs 0
quad assign abs _1 2 _3 4
comment dyadic residue: A residue B = B mod A
quad assign 3 residue 7
quad assign 5 residue 13
quad assign 4 residue 12
quad assign 3 residue _7
comment scalar extension
quad assign 10 residue 23 45 67
quad assign 2 residue iota 8
",
    },
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
quad assign A + 3
comment Another comment
B assign 10 20 30
quad assign +/ B comment sum of vector
comment Final comment at end
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
quad assign I
I assign I - 1
goto (I)/LOOP
quad assign 0
",
    },
    Demo {
        name: "Cup & Cap",
        description: "Set operations: unique, union, intersection",
        source: "\
comment monadic cup: unique (remove duplicates)
quad assign cup 1 2 3 2 1 4
quad assign cup 5 5 5
comment dyadic cup: union
quad assign 1 2 3 cup 3 4 5
quad assign 1 2 cup 1 2
comment dyadic cap: intersection
quad assign 1 2 3 4 cap 2 4 6
quad assign 1 2 3 cap 4 5 6
comment character unique
quad assign cup 'mississippi'
comment character union
quad assign 'abc' cup 'cde'
comment character intersection
quad assign 'abcabc' cap 'bcd'
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
        name: "Enclose",
        description: "Wrap a value in a box (monadic enclose)",
        source: "\
comment enclose: wrap value in a box
quad assign enclose 1 2 3
quad assign enclose 'hello'
comment round-trip: 1 pick undoes enclose
quad assign 1 pick enclose 1 2 3
quad assign 1 pick enclose 'world'
comment rho of enclosed is 1
quad assign rho enclose 1 2 3
",
    },
    Demo {
        name: "Encode & Decode",
        description: "Radix conversion: represent and evaluate",
        source: "\
comment encode (represent in radix) and decode (evaluate from radix)
comment binary: 5 in base 2
quad assign 2 2 2 encode 5
comment decode back
quad assign 2 2 2 decode 1 0 1
comment time: 3661 seconds = 1h 1m 1s
quad assign 24 60 60 encode 3661
quad assign 24 60 60 decode 1 1 1
comment base 10 digits of 123
quad assign 10 10 10 encode 123
quad assign 10 10 10 decode 1 2 3
comment round-trip
quad assign 2 2 2 2 decode 2 2 2 2 encode 13
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
rho fmt _5
rho fmt 1 2 3
'ROUND ' cat fmt 5
'Score: ' cat fmt 123
",
    },
    Demo {
        name: "Grade Up & Down",
        description: "Sort indices ascending and descending",
        source: "\
comment gradeup and gradedown (sort indices)
quad assign gradeup 30 10 20
quad assign gradedown 30 10 20
quad assign gradeup 5 3 1 4 2
comment use grade to sort: V[gradeup V]
V assign 30 10 50 20 40
quad assign V[gradeup V]
quad assign V[gradedown V]
",
    },
    Demo {
        name: "Horse Race A",
        description: "Verbose race: 5 horses, round counter, capped track display",
        source: "\
comment Horse Race A -- verbose version (from race.apl)
comment 5 named horses race to FINISH line with track visualization.
comment RACE defines POS and RND as locals, calls niladic SHOW which
comment   reads POS through dynamic scoping. Track length capped at 20.
comment Winner is horse with highest position when any horse finishes.
comment Type RACE to run again (PRNG state changes each run).
quad-seed assign 7
NH assign 5
FINISH assign 15
NAMES assign 'Lucky  ' 'Thunder' 'Shadow ' 'Comet  ' 'Blaze  '
comment
del R assign SHOW;I
R assign 0
I assign 1
LP: quad assign (I pick NAMES) cat ':' cat (((I pick POS) floor 20) rho '#')
I assign I + 1
goto (I <= NH)/LP
del
comment
del R assign RACE;POS;RND
POS assign NH rho 0
RND assign 0
quad assign 'THE RACE IS ON!'
NXT: RND assign RND + 1
quad assign '--- Round ' cat (fmt RND) cat ' ---'
POS assign POS + roll NH rho 3
SHOW
goto (or/ POS >= FINISH)/DONE
goto NXT
DONE: quad assign 'WINNER: ' cat ((1 pick (POS = ceil/ POS) compress iota NH) pick NAMES)
del
comment
RACE
",
    },
    Demo {
        name: "Horse Race B",
        description: "Idiomatic race: compact APL, minimal code",
        source: "\
comment Horse Race B -- idiomatic version (from idiomatic-race.apl)
comment Same 5 horses, but compact APL style: no round counter, no
comment   track cap, fewer lines. RACE calls niladic SHOW. POS is
comment   local to RACE, visible to SHOW via dynamic scope.
comment Type RACE to run again.
quad-seed assign 7
NAMES assign 'Lucky  ' 'Thunder' 'Shadow ' 'Comet  ' 'Blaze  '
NH assign 5
comment
del R assign SHOW;I
R assign 0
I assign 1
N: quad assign (I pick NAMES) cat ':' cat ((I pick POS) rho '#')
I assign I + 1
goto (I <= NH)/N
del
comment
del R assign RACE;POS
POS assign NH rho 0
quad assign 'THE RACE IS ON!'
L: POS assign POS + roll NH rho 3
SHOW
quad assign ''
goto (0 = or/ POS >= 15)/L
quad assign 'WINNER: ' cat (1 pick (POS = ceil/ POS) compress iota NH) pick NAMES
del
comment
RACE
",
    },
    Demo {
        name: "Index-Of & Member",
        description: "Dyadic iota (index-of) and membership test",
        source: "\
comment dyadic iota (index-of): A iota B -> position of B in A
quad assign 10 20 30 iota 20
quad assign 10 20 30 iota 40
quad assign 10 20 30 iota 10 30
comment with 0-origin
quad-origin assign 0
quad assign 10 20 30 iota 20
quad assign 10 20 30 iota 40
quad-origin assign 1
comment monadic iota still works
quad assign iota 5
comment member: A member B -> 1 where A elements in B
quad assign 1 2 3 4 member 2 4 6
quad assign 5 member 1 2 3 4 5
quad assign 5 member 1 2 3
comment character member
quad assign 'hello' member 'aeiou'
",
    },
    Demo {
        name: "Inner Product",
        description: "Matrix and vector inner products (f.g)",
        source: "\
comment inner product: A f.g B
comment dot product (sum of products)
quad assign 1 2 3 +.* 4 5 6
comment matrix multiply
quad assign (2 2 rho 1 2 3 4) +.* (2 2 rho 5 6 7 8)
comment matrix-vector multiply
quad assign (2 3 rho 1 2 3 4 5 6) +.* 1 2 3
comment boolean: any match (or.=)
quad assign 1 2 3 4 5 or.= 3 3 3 3 3
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
comment No WS FULL even with huge arguments
rho iota 999999
5 take iota 999999
5 take 5 drop iota 999999
",
    },
    Demo {
        name: "Local Variables",
        description: "Function-local scope with ;VAR syntax",
        source: "\
comment Local variables in functions (;VAR syntax)
comment Local does not pollute global scope
G assign 99
del R assign TEST X;L
L assign X * 2
R assign L + 1
del
quad assign TEST 5
quad assign G
comment G is still 99
comment Multiple locals
del R assign MULTI X;A;B;C
A assign X + 1
B assign X + 2
C assign A * B
R assign C
del
quad assign MULTI 3
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
[3] quad assign N
[4] goto (5 - N)/LOOP
[5] quad assign 99
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
        name: "Niladic Functions",
        description: "Zero-argument user-defined functions",
        source: "\
comment Niladic function test
del R assign HELLO
R assign 42
del
quad assign HELLO
comment Niladic with quad output inside
del R assign GREET
quad assign 'hello world'
R assign 0
del
quad assign GREET
comment Monadic still works
del R assign DOUBLE X
R assign X + X
del
quad assign DOUBLE 5
",
    },
    Demo {
        name: "OR & AND Reduce",
        description: "Logical or/ and and/ reduction",
        source: "\
comment or/ reduce tests
quad assign or/ 0 0 1 0
quad assign or/ 0 0 0 0
quad assign or/ 1 1 1 1
comment and/ reduce tests
quad assign and/ 1 1 1 1
quad assign and/ 1 1 0 1
quad assign and/ 0 0 0 0
comment mixed with expressions
A assign 0 1 0 1
quad assign or/ A
quad assign and/ A
comment single element
quad assign or/ 1
quad assign and/ 0
",
    },
    Demo {
        name: "Outer Product",
        description: "Outer product tables (A outer.f B)",
        source: "\
comment outer product: A outer.f B
comment addition table
quad assign 1 2 3 outer.+ 10 20
comment multiplication table
quad assign 1 2 3 4 outer.* 1 2 3 4
comment identity-like matrix
quad assign 1 2 3 outer.= 1 2 3
comment comparison
quad assign 1 2 3 outer.<= 1 2 3
",
    },
    Demo {
        name: "Power",
        description: "Integer exponentiation",
        source: "\
comment power: integer exponentiation
quad assign 2 power 10
quad assign 3 power 0
quad assign 5 power 3
comment scalar extension
quad assign 2 power 0 1 2 3 4 5 6 7 8
comment vector power vector
quad assign 1 2 3 4 5 power 5 4 3 2 1
",
    },
    Demo {
        name: "Quad I/O",
        description: "Explicit printing with quad in functions",
        source: "\
quad assign 42
quad assign 1 2 3
quad assign 'hello'
A assign 10
quad assign A + 5
quad assign iota 5
quad assign 99
del R assign SHOW X
R assign X
quad assign 'value:'
quad assign X
del
SHOW 7
",
    },
    Demo {
        name: "Quad-Origin",
        description: "Index origin: switch between 1-based and 0-based",
        source: "\
comment quad-origin: index origin system variable
comment default is 1-origin
quad assign quad-origin
quad assign iota 5
comment switch to 0-origin
quad-origin assign 0
quad assign quad-origin
quad assign iota 5
comment restore 1-origin
quad-origin assign 1
quad assign iota 5
",
    },
    Demo {
        name: "Quad-Seed",
        description: "PRNG seed for reproducible random sequences",
        source: "\
comment quad-seed: PRNG seed system variable
quad-seed assign 42
quad assign quad-seed
quad assign roll 6
quad assign roll 6
comment reseed to same value gives same sequence
quad-seed assign 42
quad assign roll 6
quad assign roll 6
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
        name: "Rotate",
        description: "Dyadic rev rotates a vector left by N positions",
        source: "\
comment dyadic rev (rotate): N rev V rotates left by N
quad assign 2 rev 1 2 3 4 5
quad assign _1 rev 1 2 3 4 5
quad assign 0 rev 1 2 3 4 5
quad assign 5 rev 1 2 3 4 5
comment character rotate
quad assign 2 rev 'abcde'
comment monadic rev (reverse) still works
quad assign rev 1 2 3 4 5
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
        name: "Signum, Factorial & Binomial",
        description: "Sign function, n!, and C(n,k) combinations",
        source: "\
comment monadic signum: _1, 0, or 1
quad assign signum _5
quad assign signum 0
quad assign signum 7
quad assign signum _3 0 4 _1
comment monadic factorial
quad assign factorial 0
quad assign factorial 1
quad assign factorial 5
quad assign factorial 7
comment dyadic binomial: K binomial N = C(N,K)
quad assign 2 binomial 5
quad assign 0 binomial 7
quad assign 3 binomial 3
quad assign 1 binomial 10
comment scalar extension
quad assign 2 binomial 3 4 5 6
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
        name: "Transpose",
        description: "Swap rows and columns of a matrix",
        source: "\
comment transpose: swap rows and columns of matrix
quad assign 2 3 rho iota 6
quad assign transpose 2 3 rho iota 6
comment 3x2 becomes 2x3
quad assign rho transpose 2 3 rho iota 6
comment scalar and vector: no-op
quad assign transpose 42
quad assign transpose 1 2 3
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
    Demo {
        name: "Without",
        description: "Set difference: elements in A not in B",
        source: "\
comment without: A without B removes elements of B from A
quad assign 1 2 3 4 5 without 2 4
quad assign 1 2 3 4 5 without 6 7
quad assign 1 2 3 4 5 without 1 2 3 4 5
comment character without
quad assign 'hello world' without 'aeiou'
quad assign 'abcdef' without 'bdf'
",
    },
];
