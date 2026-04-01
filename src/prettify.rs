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
        literate_dyadic: None,
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
        ascii: "qled",
        glyph: "\u{2395}LED", // ⎕LED
        literate_monadic: "LED",
        literate_dyadic: None,
    },
    KeywordEntry {
        ascii: "qsw",
        glyph: "\u{2395}SW", // ⎕SW
        literate_monadic: "switch",
        literate_dyadic: None,
    },
    KeywordEntry {
        ascii: "qsvo",
        glyph: "\u{2395}SVO", // ⎕SVO
        literate_monadic: "share",
        literate_dyadic: None,
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
