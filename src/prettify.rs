//! APL glyph prettification — maps ASCII keywords to APL symbols or literate names.

/// Display mode for APL output.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum DisplayMode {
    /// Show raw ASCII source (`rho`, `iota`, etc.)
    #[default]
    Repr,
    /// Show APL glyphs (`⍴`, `⍳`, `↑`, `↓`)
    Glyph,
    /// Show literate English names (`shape-of`, `index-gen`, etc.)
    Literate,
}

/// A segment of prettified output.
#[derive(Clone, Debug)]
pub enum Segment {
    /// Plain text, not a keyword.
    Plain(String),
    /// A keyword that was transformed.
    Keyword(String),
}

pub struct KeywordEntry {
    pub ascii: &'static str,
    pub glyph: &'static str,
    /// Literate name when used monadically (prefix): `rho X` → "shape-of"
    pub literate_monadic: &'static str,
    /// Literate name when used dyadically (infix): `A rho B` → "reshape".
    /// `None` means the keyword is always monadic.
    pub literate_dyadic: Option<&'static str>,
}

pub const KEYWORDS: &[KeywordEntry] = &[
    KeywordEntry {
        ascii: "rho",
        glyph: "\u{2374}", // ⍴
        literate_monadic: "shape-of",
        literate_dyadic: Some("reshape"),
    },
    KeywordEntry {
        ascii: "iota",
        glyph: "\u{2373}", // ⍳
        literate_monadic: "index-gen",
        literate_dyadic: Some("index-of"),
    },
    KeywordEntry {
        ascii: "take",
        glyph: "\u{2191}", // ↑
        literate_monadic: "take",
        literate_dyadic: None,
    },
    KeywordEntry {
        ascii: "drop",
        glyph: "\u{2193}", // ↓
        literate_monadic: "drop",
        literate_dyadic: None,
    },
    KeywordEntry {
        ascii: "qsvo",
        glyph: "\u{2395}SVO", // ⎕SVO
        literate_monadic: "share",
        literate_dyadic: Some("share"),
    },
    KeywordEntry {
        ascii: "cup",
        glyph: "\u{222A}", // ∪
        literate_monadic: "unique",
        literate_dyadic: Some("union"),
    },
    KeywordEntry {
        ascii: "cap",
        glyph: "\u{2229}", // ∩
        literate_monadic: "intersection",
        literate_dyadic: Some("intersection"),
    },
    KeywordEntry {
        ascii: "rev",
        glyph: "\u{233D}", // ⌽
        literate_monadic: "reverse",
        literate_dyadic: None,
    },
    KeywordEntry {
        ascii: "cat",
        glyph: "\u{002C}", // ,
        literate_monadic: "ravel",
        literate_dyadic: Some("catenate"),
    },
    KeywordEntry {
        ascii: "and",
        glyph: "\u{2227}", // ∧
        literate_monadic: "and",
        literate_dyadic: None,
    },
    KeywordEntry {
        ascii: "or",
        glyph: "\u{2228}", // ∨
        literate_monadic: "or",
        literate_dyadic: None,
    },
    KeywordEntry {
        ascii: "not",
        glyph: "\u{223C}", // ∼
        literate_monadic: "complement",
        literate_dyadic: None,
    },
    KeywordEntry {
        ascii: "goto",
        glyph: "\u{2192}", // →
        literate_monadic: "branch",
        literate_dyadic: None,
    },
    KeywordEntry {
        ascii: "compress",
        glyph: "/",
        literate_monadic: "compress",
        literate_dyadic: Some("compress"),
    },
    KeywordEntry {
        ascii: "ceil",
        glyph: "\u{2308}", // ⌈
        literate_monadic: "ceiling",
        literate_dyadic: Some("max"),
    },
    KeywordEntry {
        ascii: "floor",
        glyph: "\u{230A}", // ⌊
        literate_monadic: "floor",
        literate_dyadic: Some("min"),
    },
    KeywordEntry {
        ascii: "del",
        glyph: "\u{2207}", // ∇
        literate_monadic: "define-function",
        literate_dyadic: None,
    },
    KeywordEntry {
        ascii: "pick",
        glyph: "\u{2283}", // ⊃
        literate_monadic: "disclose",
        literate_dyadic: Some("pick"),
    },
    KeywordEntry {
        ascii: "roll",
        glyph: "?",
        literate_monadic: "roll",
        literate_dyadic: None,
    },
    KeywordEntry {
        ascii: "fmt",
        glyph: "\u{2355}", // ⍕
        literate_monadic: "format",
        literate_dyadic: None,
    },
    KeywordEntry {
        ascii: "abs",
        glyph: "|",
        literate_monadic: "absolute-value",
        literate_dyadic: None,
    },
    KeywordEntry {
        ascii: "residue",
        glyph: "|",
        literate_monadic: "modulo",
        literate_dyadic: Some("modulo"),
    },
    KeywordEntry {
        ascii: "signum",
        glyph: "\u{00D7}", // × (monadic × = signum)
        literate_monadic: "sign",
        literate_dyadic: None,
    },
    KeywordEntry {
        ascii: "factorial",
        glyph: "!",
        literate_monadic: "factorial",
        literate_dyadic: None,
    },
    KeywordEntry {
        ascii: "binomial",
        glyph: "!",
        literate_monadic: "combinations",
        literate_dyadic: Some("combinations"),
    },
    KeywordEntry {
        ascii: "member",
        glyph: "\u{2208}", // ∈
        literate_monadic: "membership",
        literate_dyadic: Some("membership"),
    },
    // Hyphenated quad-* keywords must precede `quad` for longest-match-first.
    KeywordEntry {
        ascii: "quad-origin",
        glyph: "\u{2395}IO", // ⎕IO
        literate_monadic: "index-origin",
        literate_dyadic: None,
    },
    KeywordEntry {
        ascii: "quad-seed",
        glyph: "\u{2395}RL", // ⎕RL
        literate_monadic: "random-seed",
        literate_dyadic: None,
    },
    KeywordEntry {
        ascii: "quad",
        glyph: "\u{2395}", // ⎕ (bare quad I/O)
        literate_monadic: "print",
        literate_dyadic: None,
    },
    KeywordEntry {
        ascii: "assign",
        glyph: "\u{2190}", // ←
        literate_monadic: "assign",
        literate_dyadic: None,
    },
    KeywordEntry {
        ascii: "comment",
        glyph: "\u{235D}", // ⍝
        literate_monadic: "comment",
        literate_dyadic: None,
    },
];

/// Prettify a line according to the display mode.
///
/// Returns a list of segments: `Plain` for non-keyword text, `Keyword` for
/// transformed keywords. In `Repr` mode, returns a single `Plain` segment.
pub fn prettify_line(line: &str, mode: DisplayMode) -> Vec<Segment> {
    if mode == DisplayMode::Repr {
        return vec![Segment::Plain(line.to_string())];
    }

    let mut segments = Vec::new();
    let mut i = 0;
    let bytes = line.as_bytes();
    let mut plain_start = 0;

    while i < bytes.len() {
        // Only try keyword match if we're at a word boundary (start of line or
        // preceded by a non-alpha character) AND current char is lowercase alpha.
        if bytes[i].is_ascii_lowercase()
            && (i == 0 || !bytes[i - 1].is_ascii_alphanumeric())
            && let Some((entry, len)) = try_match_keyword(&bytes[i..])
        {
            // Check the char after the keyword is not alphanumeric (word boundary).
            let end = i + len;
            if end >= bytes.len() || !bytes[end].is_ascii_alphanumeric() {
                // Flush preceding plain text.
                if plain_start < i {
                    segments.push(Segment::Plain(line[plain_start..i].to_string()));
                }
                let replacement = match mode {
                    DisplayMode::Glyph => entry.glyph.to_string(),
                    DisplayMode::Literate => {
                        // Dyadic if there's a non-whitespace token before the keyword.
                        let is_dyadic = entry.literate_dyadic.is_some()
                            && line[..i].contains(|c: char| !c.is_ascii_whitespace());
                        if is_dyadic {
                            entry.literate_dyadic.unwrap().to_string()
                        } else {
                            entry.literate_monadic.to_string()
                        }
                    }
                    DisplayMode::Repr => unreachable!(),
                };
                segments.push(Segment::Keyword(replacement));
                i = end;
                plain_start = i;

                // After `comment`, the rest of the line is comment text —
                // preserve it literally, no further keyword matching.
                if entry.ascii == "comment" {
                    if plain_start < bytes.len() {
                        segments.push(Segment::Plain(line[plain_start..].to_string()));
                    }
                    return segments;
                }

                continue;
            }
        }
        i += 1;
    }

    // Flush remaining plain text.
    if plain_start < bytes.len() {
        segments.push(Segment::Plain(line[plain_start..].to_string()));
    }

    segments
}

fn try_match_keyword(bytes: &[u8]) -> Option<(&'static KeywordEntry, usize)> {
    for entry in KEYWORDS {
        let kw = entry.ascii.as_bytes();
        if bytes.len() >= kw.len() && &bytes[..kw.len()] == kw {
            return Some((entry, kw.len()));
        }
    }
    None
}

/// Literate-name-to-ASCII mapping entry, sorted longest-first for matching.
struct LiterateEntry {
    literate: &'static str,
    ascii: &'static str,
}

/// Build the literate→ASCII lookup table (longest match first).
fn literate_entries() -> Vec<LiterateEntry> {
    let mut entries = Vec::new();
    for kw in KEYWORDS {
        // Always add monadic literate name.
        if kw.literate_monadic != kw.ascii {
            entries.push(LiterateEntry {
                literate: kw.literate_monadic,
                ascii: kw.ascii,
            });
        }
        // Add dyadic literate name if it differs from ascii and monadic.
        if let Some(dyadic) = kw.literate_dyadic
            && dyadic != kw.ascii
            && dyadic != kw.literate_monadic
        {
            entries.push(LiterateEntry {
                literate: dyadic,
                ascii: kw.ascii,
            });
        }
    }
    // Sort longest-first to avoid partial matches.
    entries.sort_by(|a, b| b.literate.len().cmp(&a.literate.len()));
    entries
}

/// Translate literate names in input to their ASCII keyword equivalents.
///
/// For example, `"shape-of 1 2 3"` → `"rho 1 2 3"`, `"3 reshape 4 5 6"` → `"3 rho 4 5 6"`.
pub fn translate_literate_to_ascii(input: &str) -> String {
    let entries = literate_entries();
    if entries.is_empty() {
        return input.to_string();
    }

    let bytes = input.as_bytes();
    let mut result = String::with_capacity(input.len());
    let mut i = 0;

    while i < bytes.len() {
        if let Some((entry, len)) = try_match_literate(&entries, &bytes[i..]) {
            // Check word boundaries: preceding char must not be alphanumeric/hyphen,
            // following char must not be alphanumeric/hyphen.
            let at_start = i == 0 || !is_word_char(bytes[i - 1]);
            let at_end = i + len >= bytes.len() || !is_word_char(bytes[i + len]);
            if at_start && at_end {
                result.push_str(entry.ascii);
                i += len;
                continue;
            }
        }
        result.push(bytes[i] as char);
        i += 1;
    }

    result
}

fn is_word_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'-'
}

/// Entry in the glyph-to-ASCII reverse mapping table.
struct GlyphEntry {
    glyph: &'static str,
    ascii: &'static str,
    /// If true, insert spaces around the replacement when adjacent to alphanumerics.
    /// Needed for multi-char alphabetic keywords (rho, iota, assign) but not for operators (*, /).
    pad: bool,
}

/// Glyph-to-ASCII mapping, longest glyphs first to avoid partial matches.
const GLYPH_MAP: &[GlyphEntry] = &[
    // Multi-char quad glyphs (longest first)
    GlyphEntry {
        glyph: "\u{2395}SVO",
        ascii: "qsvo",
        pad: true,
    },
    GlyphEntry {
        glyph: "\u{2395}RL",
        ascii: "quad-seed",
        pad: true,
    }, // ⎕RL
    GlyphEntry {
        glyph: "\u{2395}IO",
        ascii: "quad-origin",
        pad: true,
    }, // ⎕IO
    GlyphEntry {
        glyph: "\u{2395}\u{2190}",
        ascii: "quad assign",
        pad: true,
    }, // ⎕← quad output
    GlyphEntry {
        glyph: "\u{2395}",
        ascii: "quad",
        pad: true,
    }, // ⎕ bare quad
    // Single-char APL glyphs → keywords
    GlyphEntry {
        glyph: "\u{2374}",
        ascii: "rho",
        pad: true,
    }, // ⍴
    GlyphEntry {
        glyph: "\u{2373}",
        ascii: "iota",
        pad: true,
    }, // ⍳
    GlyphEntry {
        glyph: "\u{2191}",
        ascii: "take",
        pad: true,
    }, // ↑
    GlyphEntry {
        glyph: "\u{2193}",
        ascii: "drop",
        pad: true,
    }, // ↓
    GlyphEntry {
        glyph: "\u{233D}",
        ascii: "rev",
        pad: true,
    }, // ⌽
    GlyphEntry {
        glyph: "\u{2227}",
        ascii: "and",
        pad: true,
    }, // ∧
    GlyphEntry {
        glyph: "\u{2228}",
        ascii: "or",
        pad: true,
    }, // ∨
    GlyphEntry {
        glyph: "\u{223C}",
        ascii: "not",
        pad: true,
    }, // ∼
    GlyphEntry {
        glyph: "\u{2192}",
        ascii: "goto",
        pad: true,
    }, // →
    GlyphEntry {
        glyph: "\u{2308}",
        ascii: "ceil",
        pad: true,
    }, // ⌈
    GlyphEntry {
        glyph: "\u{230A}",
        ascii: "floor",
        pad: true,
    }, // ⌊
    GlyphEntry {
        glyph: "\u{2207}",
        ascii: "del",
        pad: true,
    }, // ∇
    // Newer single-char APL glyphs → keywords
    GlyphEntry {
        glyph: "\u{2283}",
        ascii: "pick",
        pad: true,
    }, // ⊃ pick/disclose
    GlyphEntry {
        glyph: "\u{2355}",
        ascii: "fmt",
        pad: true,
    }, // ⍕ format
    GlyphEntry {
        glyph: "\u{235D}",
        ascii: "comment",
        pad: true,
    }, // ⍝ comment
    GlyphEntry {
        glyph: "\u{222A}",
        ascii: "cup",
        pad: true,
    }, // ∪ unique/union
    GlyphEntry {
        glyph: "\u{2229}",
        ascii: "cap",
        pad: true,
    }, // ∩ intersection
    GlyphEntry {
        glyph: "\u{2208}",
        ascii: "member",
        pad: true,
    }, // ∈ membership
    // Note: `?`, `!`, `|` are NOT in the glyph map because they are plain
    // ASCII characters that appear in string literals and regular text.
    // They only exist as keywords (roll, factorial, abs) in the KEYWORDS table.
    // Non-keyword APL characters → ASCII operators (no padding needed)
    GlyphEntry {
        glyph: "\u{2190}",
        ascii: "assign",
        pad: true,
    }, // ← assignment
    GlyphEntry {
        glyph: "\u{00D7}",
        ascii: "*",
        pad: false,
    }, // × multiply
    GlyphEntry {
        glyph: "\u{00F7}",
        ascii: "/",
        pad: false,
    }, // ÷ divide
    GlyphEntry {
        glyph: "\u{00AF}",
        ascii: "_",
        pad: false,
    }, // ¯ high minus (negative prefix)
];

/// Translate APL Unicode glyphs to COR24 ASCII keywords.
///
/// Converts a real APL source file (e.g. from GNU APL) into our ASCII dialect.
/// Single-char glyphs like `⍴` become keywords like `rho` with space padding
/// to ensure proper word boundaries. Characters like `←` become `assign`.
pub fn translate_glyph_to_ascii(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if let Some((entry, char_count)) = try_match_glyph(&chars[i..]) {
            if entry.pad {
                // Add space before if preceded by alphanumeric
                if !result.is_empty() && result.ends_with(|c: char| c.is_alphanumeric()) {
                    result.push(' ');
                }
                result.push_str(entry.ascii);
                // Add space after if followed by alphanumeric
                if i + char_count < chars.len() && chars[i + char_count].is_alphanumeric() {
                    result.push(' ');
                }
            } else {
                result.push_str(entry.ascii);
            }
            i += char_count;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }

    result
}

fn try_match_glyph(chars: &[char]) -> Option<(&'static GlyphEntry, usize)> {
    // Collect chars into a string for matching (up to max glyph length of 4)
    let max_len = chars.len().min(4);
    let window: String = chars[..max_len].iter().collect();
    for entry in GLYPH_MAP {
        if window.starts_with(entry.glyph) {
            return Some((entry, entry.glyph.chars().count()));
        }
    }
    None
}

fn try_match_literate<'a>(
    entries: &'a [LiterateEntry],
    bytes: &[u8],
) -> Option<(&'a LiterateEntry, usize)> {
    for entry in entries {
        let lit = entry.literate.as_bytes();
        if bytes.len() >= lit.len() && bytes[..lit.len()].eq_ignore_ascii_case(lit) {
            return Some((entry, lit.len()));
        }
    }
    None
}
