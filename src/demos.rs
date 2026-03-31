//! Demo registry — APL programs fed as text via UART to the interpreter.

pub struct Demo {
    pub name: &'static str,
    pub description: &'static str,
    pub source: &'static str,
}

/// Demo programs in COR24 APL ASCII syntax.
/// Each line is sent to the interpreter via UART as if the user typed it.
pub const DEMOS: &[Demo] = &[
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
