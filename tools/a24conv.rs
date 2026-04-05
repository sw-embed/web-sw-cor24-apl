//! a24conv — Convert between APL Unicode (.apl) and COR24 ASCII (.a24) formats.
//!
//! Usage:
//!   a24conv to-a24 input.apl [output.a24]   Convert APL glyphs to ASCII keywords
//!   a24conv to-apl input.a24 [output.apl]   Convert ASCII keywords to APL glyphs
//!
//! If no output file is given, writes to stdout.
//!
//! Build:
//!   rustc tools/a24conv.rs -o tools/a24conv

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

// --- Glyph-to-ASCII mapping (APL Unicode → COR24 ASCII keywords) ---

struct GlyphEntry {
    glyph: &'static str,
    ascii: &'static str,
    /// If true, insert spaces around the replacement when adjacent to alphanumerics.
    pad: bool,
}

const GLYPH_MAP: &[GlyphEntry] = &[
    // Multi-char quad glyphs (longest first)
    GlyphEntry { glyph: "\u{2395}SVO", ascii: "qsvo",        pad: true },
    GlyphEntry { glyph: "\u{2395}RL",  ascii: "quad-seed",   pad: true },  // ⎕RL
    GlyphEntry { glyph: "\u{2395}IO",  ascii: "quad-origin", pad: true },  // ⎕IO
    GlyphEntry { glyph: "\u{2395}\u{2190}", ascii: "quad assign", pad: true }, // ⎕←
    GlyphEntry { glyph: "\u{2395}",    ascii: "quad",        pad: true },  // ⎕ bare quad
    // Single-char APL glyphs → keywords
    GlyphEntry { glyph: "\u{2374}", ascii: "rho",      pad: true },  // ⍴
    GlyphEntry { glyph: "\u{2373}", ascii: "iota",     pad: true },  // ⍳
    GlyphEntry { glyph: "\u{2191}", ascii: "take",     pad: true },  // ↑
    GlyphEntry { glyph: "\u{2193}", ascii: "drop",     pad: true },  // ↓
    GlyphEntry { glyph: "\u{233D}", ascii: "rev",      pad: true },  // ⌽
    GlyphEntry { glyph: "\u{2227}", ascii: "and",      pad: true },  // ∧
    GlyphEntry { glyph: "\u{2228}", ascii: "or",       pad: true },  // ∨
    GlyphEntry { glyph: "\u{223C}", ascii: "not",      pad: true },  // ∼
    GlyphEntry { glyph: "\u{2192}", ascii: "goto",     pad: true },  // →
    GlyphEntry { glyph: "\u{2308}", ascii: "ceil",     pad: true },  // ⌈
    GlyphEntry { glyph: "\u{230A}", ascii: "floor",    pad: true },  // ⌊
    GlyphEntry { glyph: "\u{2207}", ascii: "del",      pad: true },  // ∇
    GlyphEntry { glyph: "\u{002C}", ascii: "cat",      pad: true },  // , (catenate)
    GlyphEntry { glyph: "\u{2283}", ascii: "pick",     pad: true },  // ⊃
    GlyphEntry { glyph: "\u{2355}", ascii: "fmt",      pad: true },  // ⍕
    GlyphEntry { glyph: "\u{235D}", ascii: "comment",  pad: true },  // ⍝
    GlyphEntry { glyph: "\u{222A}", ascii: "cup",      pad: true },  // ∪ unique/union
    GlyphEntry { glyph: "\u{2229}", ascii: "cap",      pad: true },  // ∩ intersection
    GlyphEntry { glyph: "\u{220A}", ascii: "member",   pad: true },  // ∊ membership
    GlyphEntry { glyph: "\u{234B}", ascii: "gradeup",  pad: true },  // ⍋
    GlyphEntry { glyph: "\u{2352}", ascii: "gradedown",pad: true },  // ⍒
    GlyphEntry { glyph: "\u{2349}", ascii: "transpose",pad: true },  // ⍉
    GlyphEntry { glyph: "\u{22A4}", ascii: "encode",   pad: true },  // ⊤
    GlyphEntry { glyph: "\u{22A5}", ascii: "decode",   pad: true },  // ⊥
    GlyphEntry { glyph: "\u{2282}", ascii: "enclose",  pad: true },  // ⊂
    GlyphEntry { glyph: "\u{22C6}", ascii: "power",    pad: true },  // ⋆
    // Note: ?, !, | omitted — plain ASCII chars that appear in strings
    // Non-keyword APL characters → ASCII operators
    GlyphEntry { glyph: "\u{2190}", ascii: "assign", pad: true },  // ← assignment
    GlyphEntry { glyph: "\u{00D7}", ascii: "*",  pad: false },  // × multiply
    GlyphEntry { glyph: "\u{00F7}", ascii: "/",  pad: false },  // ÷ divide
    GlyphEntry { glyph: "\u{00AF}", ascii: "_",  pad: false },  // ¯ high minus
];

// --- ASCII-to-Glyph mapping (COR24 ASCII keywords → APL Unicode) ---

struct KeywordEntry {
    ascii: &'static str,
    glyph: &'static str,
}

const KEYWORD_MAP: &[KeywordEntry] = &[
    // Longest first to avoid partial matches (hyphenated before plain)
    KeywordEntry { ascii: "quad-origin", glyph: "\u{2395}IO" },
    KeywordEntry { ascii: "quad-seed",   glyph: "\u{2395}RL" },
    KeywordEntry { ascii: "transpose",   glyph: "\u{2349}" }, // ⍉
    KeywordEntry { ascii: "gradedown",   glyph: "\u{2352}" }, // ⍒
    KeywordEntry { ascii: "factorial",   glyph: "!" },
    KeywordEntry { ascii: "binomial",    glyph: "!" },
    KeywordEntry { ascii: "compress",    glyph: "/" },
    KeywordEntry { ascii: "comment",     glyph: "\u{235D}" }, // ⍝
    KeywordEntry { ascii: "without",     glyph: "~" },
    KeywordEntry { ascii: "gradeup",     glyph: "\u{234B}" }, // ⍋
    KeywordEntry { ascii: "enclose",     glyph: "\u{2282}" }, // ⊂
    KeywordEntry { ascii: "residue",     glyph: "|" },
    KeywordEntry { ascii: "signum",      glyph: "\u{00D7}" }, // ×
    KeywordEntry { ascii: "encode",      glyph: "\u{22A4}" }, // ⊤
    KeywordEntry { ascii: "decode",      glyph: "\u{22A5}" }, // ⊥
    KeywordEntry { ascii: "member",      glyph: "\u{220A}" }, // ∊
    KeywordEntry { ascii: "power",       glyph: "\u{22C6}" }, // ⋆
    KeywordEntry { ascii: "assign",      glyph: "\u{2190}" }, // ←
    KeywordEntry { ascii: "floor",       glyph: "\u{230A}" }, // ⌊
    KeywordEntry { ascii: "ceil",        glyph: "\u{2308}" }, // ⌈
    KeywordEntry { ascii: "goto",        glyph: "\u{2192}" }, // →
    KeywordEntry { ascii: "iota",        glyph: "\u{2373}" }, // ⍳
    KeywordEntry { ascii: "take",        glyph: "\u{2191}" }, // ↑
    KeywordEntry { ascii: "drop",        glyph: "\u{2193}" }, // ↓
    KeywordEntry { ascii: "pick",        glyph: "\u{2283}" }, // ⊃
    KeywordEntry { ascii: "roll",        glyph: "?" },
    KeywordEntry { ascii: "quad",        glyph: "\u{2395}" }, // ⎕
    KeywordEntry { ascii: "qsvo",        glyph: "\u{2395}SVO" },
    KeywordEntry { ascii: "rho",         glyph: "\u{2374}" }, // ⍴
    KeywordEntry { ascii: "rev",         glyph: "\u{233D}" }, // ⌽
    KeywordEntry { ascii: "fmt",         glyph: "\u{2355}" }, // ⍕
    KeywordEntry { ascii: "cup",         glyph: "\u{222A}" }, // ∪
    KeywordEntry { ascii: "cap",         glyph: "\u{2229}" }, // ∩
    KeywordEntry { ascii: "abs",         glyph: "|" },
    KeywordEntry { ascii: "cat",         glyph: "," },
    KeywordEntry { ascii: "and",         glyph: "\u{2227}" }, // ∧
    KeywordEntry { ascii: "not",         glyph: "\u{223C}" }, // ∼
    KeywordEntry { ascii: "del",         glyph: "\u{2207}" }, // ∇
    KeywordEntry { ascii: "or",          glyph: "\u{2228}" }, // ∨
];

/// No multi-char operator conversions needed — assign is now a keyword.
const OPERATOR_MAP: &[(&str, &str)] = &[];

fn glyph_to_ascii(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut result = String::with_capacity(input.len());
    let mut i = 0;

    while i < chars.len() {
        let max_len = chars.len().min(i + 4);
        let window: String = chars[i..max_len].iter().collect();
        let mut matched = false;

        for entry in GLYPH_MAP {
            if window.starts_with(entry.glyph) {
                if entry.pad {
                    if !result.is_empty() && result.ends_with(|c: char| c.is_alphanumeric()) {
                        result.push(' ');
                    }
                    result.push_str(entry.ascii);
                    let skip = entry.glyph.chars().count();
                    if i + skip < chars.len() && chars[i + skip].is_alphanumeric() {
                        result.push(' ');
                    }
                    i += skip;
                } else {
                    result.push_str(entry.ascii);
                    i += entry.glyph.chars().count();
                }
                matched = true;
                break;
            }
        }

        if !matched {
            result.push(chars[i]);
            i += 1;
        }
    }

    result
}

fn ascii_to_glyph(input: &str) -> String {
    let mut result = String::with_capacity(input.len());

    // First pass: convert multi-char operators (if any) to glyphs
    let mut intermediate = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        let mut matched = false;
        for &(ascii, glyph) in OPERATOR_MAP {
            let mut achars = ascii.chars();
            if achars.next() == Some(c) {
                // Check remaining chars of the operator
                let remaining: Vec<char> = achars.collect();
                let mut peek_matched = true;
                let mut peeked = Vec::new();
                for &rc in &remaining {
                    if let Some(&pc) = chars.peek() {
                        if pc == rc {
                            peeked.push(chars.next().unwrap());
                        } else {
                            peek_matched = false;
                            break;
                        }
                    } else {
                        peek_matched = false;
                        break;
                    }
                }
                if peek_matched {
                    intermediate.push_str(glyph);
                    matched = true;
                    break;
                } else {
                    // Put back peeked chars by adding them to intermediate
                    intermediate.push(c);
                    for pc in peeked {
                        intermediate.push(pc);
                    }
                    matched = true;
                    break;
                }
            }
        }
        if !matched {
            intermediate.push(c);
        }
    }

    // Second pass: convert keywords at word boundaries
    let bytes = intermediate.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i].is_ascii_lowercase()
            && (i == 0 || !bytes[i - 1].is_ascii_alphanumeric())
        {
            let mut matched = false;
            for entry in KEYWORD_MAP {
                let kw = entry.ascii.as_bytes();
                if i + kw.len() <= bytes.len()
                    && &bytes[i..i + kw.len()] == kw
                    && (i + kw.len() >= bytes.len()
                        || !bytes[i + kw.len()].is_ascii_alphanumeric())
                {
                    result.push_str(entry.glyph);
                    i += kw.len();
                    matched = true;
                    break;
                }
            }
            if !matched {
                result.push(bytes[i] as char);
                i += 1;
            }
        } else {
            // Safe: after operator substitution the remaining text is either
            // ASCII or known-good UTF-8 glyphs. Walk char-by-char.
            let ch = intermediate[i..].chars().next().unwrap();
            result.push(ch);
            i += ch.len_utf8();
        }
    }

    result
}

fn usage() {
    eprintln!("a24conv — Convert between APL Unicode (.apl) and COR24 ASCII (.a24)");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  a24conv to-a24 input.apl [output.a24]");
    eprintln!("  a24conv to-apl input.a24 [output.apl]");
    eprintln!();
    eprintln!("If no output file is given, writes to stdout.");
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        usage();
    }

    let direction = &args[1];
    let input_path = &args[2];
    let output_path = args.get(3);

    let input = fs::read_to_string(input_path).unwrap_or_else(|e| {
        eprintln!("Error reading {}: {}", input_path, e);
        process::exit(1);
    });

    let output = match direction.as_str() {
        "to-a24" => input.lines().map(glyph_to_ascii).collect::<Vec<_>>().join("\n"),
        "to-apl" => input.lines().map(ascii_to_glyph).collect::<Vec<_>>().join("\n"),
        _ => {
            eprintln!("Unknown direction '{}'. Use 'to-a24' or 'to-apl'.", direction);
            process::exit(1);
        }
    };

    // Preserve trailing newline if input had one
    let output = if input.ends_with('\n') {
        format!("{}\n", output)
    } else {
        output
    };

    if let Some(path) = output_path {
        fs::write(path, &output).unwrap_or_else(|e| {
            eprintln!("Error writing {}: {}", path, e);
            process::exit(1);
        });
        eprintln!("{} -> {}", input_path, path);
    } else {
        io::stdout().write_all(output.as_bytes()).unwrap();
    }
}
