//! APL interpreter binary — pre-assembled at build time.
//!
//! The COR24 assembler runs in build.rs, not in WASM.

/// Pre-assembled APL interpreter COR24 machine code.
pub const APL_BINARY: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/apl.bin"));
