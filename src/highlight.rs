//! Code block syntax colouring: a small lexer for the common languages,
//! coloured with a light code palette (muted red keywords, olive names,
//! ochre strings, blue types, violet numbers). Unknown languages (and
//! Plain Text) stay uncoloured.

use crate::theme::c;
use gpui::Hsla;
use std::ops::Range;

/// A coloured range; `true` = italic (comments).
pub type Span = (Range<usize>, Hsla, bool);

#[derive(Clone, Copy, PartialEq)]
enum Tok {
    Comment,
    Keyword,
    Literal,
    String,
    Number,
    BuiltIn,
    Type,
    Title,
    Tag,
    Attr,
}

/// The theme's `.token.*` colours (the light code palette by default).
fn color(t: Tok) -> Hsla {
    let key = match t {
        Tok::Comment => "syntax.comment",
        Tok::Keyword => "syntax.keyword",
        Tok::Tag => "syntax.tag",
        Tok::Literal => "syntax.literal",
        Tok::Number => "syntax.number",
        Tok::String => "syntax.string",
        Tok::Type => "syntax.type",
        Tok::Attr => "syntax.attr",
        Tok::BuiltIn | Tok::Title => "syntax.function",
    };
    crate::editor_theme::style().color(key).unwrap_or_else(|| c(0x8d9096))
}

#[derive(Default)]
struct Syntax {
    line_comments: &'static [&'static str],
    block_comment: Option<(&'static str, &'static str)>,
    quotes: &'static str,
    keywords: &'static [&'static str],
    literals: &'static [&'static str],
    builtins: &'static [&'static str],
    types: &'static [&'static str],
    /// Keywords whose following identifier is a definition name (title).
    defs: &'static [&'static str],
    /// `name!` is a macro (Rust).
    bang_macros: bool,
    /// `$var` / `@var` style variables.
    sigil_vars: &'static str,
    /// Capitalised identifiers are types.
    caps_types: bool,
    case_insensitive: bool,
    /// Python-style `f"…"` prefixes and `"""…"""` strings.
    py_strings: bool,
}

const C_LIKE_LITERALS: &[&str] = &["true", "false", "null", "nullptr", "NULL", "undefined", "nil", "None", "True", "False"];

fn syntax(lang: &str) -> Option<Syntax> {
    let s = match lang {
        "Rust" => Syntax {
            line_comments: &["//"],
            block_comment: Some(("/*", "*/")),
            quotes: "\"",
            keywords: &[
                "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "fn", "for", "if",
                "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self", "static",
                "struct", "super", "trait", "type", "unsafe", "use", "where", "while", "union", "macro_rules",
            ],
            literals: &["true", "false", "Some", "None", "Ok", "Err"],
            types: &[
                "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32", "f64", "str",
                "char", "bool", "Box", "Option", "Result", "String", "Vec",
            ],
            defs: &["fn", "struct", "enum", "trait", "mod", "type", "union"],
            bang_macros: true,
            ..Default::default()
        },
        "JavaScript" | "TypeScript" | "CoffeeScript" | "JSON" => Syntax {
            line_comments: &["//"],
            block_comment: Some(("/*", "*/")),
            quotes: "\"'`",
            keywords: &[
                "as", "async", "await", "break", "case", "catch", "class", "const", "continue", "debugger", "default", "delete",
                "do", "else", "enum", "export", "extends", "finally", "for", "from", "function", "get", "if", "implements",
                "import", "in", "instanceof", "interface", "let", "new", "of", "private", "protected", "public", "readonly",
                "return", "set", "static", "super", "switch", "this", "throw", "try", "type", "typeof", "var", "void", "while",
                "with", "yield", "declare", "namespace", "abstract", "keyof",
            ],
            literals: &["true", "false", "null", "undefined", "NaN", "Infinity"],
            builtins: &[
                "console", "window", "document", "Math", "JSON", "Promise", "Object", "Array", "String", "Number", "Boolean",
                "Symbol", "Map", "Set", "Date", "Error", "RegExp", "require", "module", "exports", "parseInt", "parseFloat",
            ],
            types: &["string", "number", "boolean", "any", "unknown", "never", "object", "bigint"],
            defs: &["function", "class", "interface"],
            ..Default::default()
        },
        "Python" => Syntax {
            line_comments: &["#"],
            quotes: "\"'",
            keywords: &[
                "and", "as", "assert", "async", "await", "break", "class", "continue", "def", "del", "elif", "else", "except",
                "finally", "for", "from", "global", "if", "import", "in", "is", "lambda", "nonlocal", "not", "or", "pass",
                "raise", "return", "try", "while", "with", "yield", "match", "case",
            ],
            literals: &["True", "False", "None"],
            builtins: &[
                "print", "len", "range", "int", "str", "float", "list", "dict", "set", "tuple", "open", "super", "isinstance",
                "enumerate", "zip", "map", "filter", "sorted", "sum", "min", "max", "abs", "type", "object", "self",
            ],
            defs: &["def", "class"],
            py_strings: true,
            ..Default::default()
        },
        "Go" => Syntax {
            line_comments: &["//"],
            block_comment: Some(("/*", "*/")),
            quotes: "\"'`",
            keywords: &[
                "break", "case", "chan", "const", "continue", "default", "defer", "else", "fallthrough", "for", "func", "go",
                "goto", "if", "import", "interface", "map", "package", "range", "return", "select", "struct", "switch", "type",
                "var",
            ],
            literals: &["true", "false", "nil", "iota"],
            builtins: &["append", "cap", "close", "copy", "delete", "len", "make", "new", "panic", "print", "println", "recover"],
            types: &[
                "bool", "byte", "error", "float32", "float64", "int", "int8", "int16", "int32", "int64", "rune", "string",
                "uint", "uint8", "uint16", "uint32", "uint64", "uintptr", "any",
            ],
            defs: &["func"],
            ..Default::default()
        },
        "Java" | "Kotlin" | "C#" | "C" | "C++" | "Objective-C" | "Swift" | "Dart" | "Scala" | "Groovy" | "D" | "Apex"
        | "Solidity" | "OpenGL Shading Language" | "ProtoBuf" | "Thrift" | "PHP" => Syntax {
            line_comments: &["//"],
            block_comment: Some(("/*", "*/")),
            quotes: "\"'",
            keywords: &[
                "abstract", "auto", "break", "case", "catch", "class", "const", "continue", "default", "delete", "do", "else",
                "enum", "explicit", "export", "extends", "extern", "final", "finally", "for", "fun", "func", "function", "goto",
                "if", "implements", "import", "include", "inline", "interface", "internal", "let", "namespace", "new",
                "object", "operator", "override", "package", "private", "protected", "public", "return", "sealed", "sizeof",
                "static", "struct", "switch", "template", "this", "throw", "throws", "try", "typedef", "typename", "union",
                "using", "val", "var", "virtual", "volatile", "while", "when", "yield", "async", "await", "guard", "in", "is",
                "as", "self", "super", "message", "service", "rpc", "returns", "contract", "pragma", "echo",
            ],
            literals: C_LIKE_LITERALS,
            types: &[
                "int", "long", "short", "char", "float", "double", "void", "bool", "boolean", "byte", "unsigned", "signed",
                "string", "String", "Int", "Double", "Float", "Bool", "Unit", "size_t", "uint8_t", "uint32_t", "int32_t",
                "int64_t", "uint64_t", "vec2", "vec3", "vec4", "mat4", "int32", "int64", "uint32", "uint64",
            ],
            defs: &["class", "struct", "interface", "enum", "fun", "func", "function", "message", "service", "contract"],
            sigil_vars: if lang == "PHP" { "$" } else { "" },
            ..Default::default()
        },
        "Bash" | "Shell" | "PowerShell" | "Dockerfile" | "Makefile" | "CMake" | "Nginx" | "Apache" | "Perl" => Syntax {
            line_comments: &["#"],
            quotes: "\"'",
            keywords: &[
                "if", "then", "else", "elif", "fi", "for", "while", "until", "do", "done", "case", "esac", "in", "function",
                "return", "local", "export", "select", "FROM", "RUN", "CMD", "COPY", "ADD", "ENV", "ARG", "WORKDIR",
                "EXPOSE", "ENTRYPOINT", "USER", "VOLUME", "LABEL", "sub", "my", "use",
            ],
            literals: &["true", "false"],
            builtins: &[
                "echo", "cd", "ls", "cat", "grep", "sed", "awk", "rm", "cp", "mv", "mkdir", "export", "source", "set", "unset",
                "printf", "read", "exit", "sudo", "chmod", "chown", "git", "npm", "curl",
            ],
            sigil_vars: "$",
            ..Default::default()
        },
        "SQL" => Syntax {
            line_comments: &["--"],
            block_comment: Some(("/*", "*/")),
            quotes: "'\"`",
            keywords: &[
                "select", "from", "where", "and", "or", "not", "insert", "into", "values", "update", "set", "delete", "create",
                "table", "drop", "alter", "add", "index", "join", "left", "right", "inner", "outer", "on", "group", "by", "order",
                "having", "limit", "offset", "as", "distinct", "union", "all", "case", "when", "then", "else", "end", "in",
                "is", "like", "between", "exists", "primary", "key", "foreign", "references", "default", "asc", "desc", "with",
            ],
            literals: &["null", "true", "false"],
            builtins: &["count", "sum", "avg", "min", "max", "coalesce", "now", "concat"],
            types: &["int", "integer", "varchar", "text", "char", "date", "datetime", "timestamp", "boolean", "decimal", "float", "bigint"],
            case_insensitive: true,
            ..Default::default()
        },
        "Ruby" => Syntax {
            line_comments: &["#"],
            quotes: "\"'",
            keywords: &[
                "alias", "and", "begin", "break", "case", "class", "def", "defined?", "do", "else", "elsif", "end", "ensure",
                "for", "if", "in", "module", "next", "not", "or", "redo", "rescue", "retry", "return", "self", "super", "then",
                "undef", "unless", "until", "when", "while", "yield", "require", "attr_accessor",
            ],
            literals: &["true", "false", "nil"],
            builtins: &["puts", "print", "p"],
            defs: &["def", "class", "module"],
            sigil_vars: "@$",
            caps_types: true,
            ..Default::default()
        },
        "Lua" => Syntax {
            line_comments: &["--"],
            quotes: "\"'",
            keywords: &[
                "and", "break", "do", "else", "elseif", "end", "for", "function", "goto", "if", "in", "local", "not", "or",
                "repeat", "return", "then", "until", "while",
            ],
            literals: &["true", "false", "nil"],
            builtins: &["print", "pairs", "ipairs", "require", "table", "string", "math", "type", "tostring", "tonumber"],
            defs: &["function"],
            ..Default::default()
        },
        "YAML" | "TOML" | "Properties" => Syntax { line_comments: &["#"], quotes: "\"'", literals: &["true", "false", "null", "yes", "no", "~"], ..Default::default() },
        "R" | "Julia" | "MATLAB" | "Haskell" | "Erlang" | "Elixir" | "Lisp" | "Scheme" | "Fortran" | "SAS" | "VBScript"
        | "Visual Basic" | "Delphi" | "Ada" | "COBOL" | "Prolog" | "VHDL" | "Verilog" | "GraphQL" | "ABAP" => Syntax {
            line_comments: match lang {
                "MATLAB" | "Erlang" | "Prolog" => &["%"],
                "Haskell" | "Ada" | "VHDL" => &["--"],
                "Lisp" | "Scheme" => &[";"],
                "Fortran" => &["!"],
                "VBScript" | "Visual Basic" => &["'"],
                "Verilog" | "Delphi" => &["//"],
                _ => &["#"],
            },
            quotes: "\"",
            keywords: &[
                "function", "end", "if", "else", "elseif", "for", "while", "return", "in", "do", "then", "module", "let", "where",
                "case", "of", "import", "begin", "procedure", "program", "var", "const", "type", "begin", "query", "mutation",
                "fragment", "Dim", "Sub", "Function", "End", "If", "Then", "Else", "For", "Next", "entity", "architecture",
                "signal", "process", "always", "assign", "wire", "reg", "input", "output", "define", "lambda",
            ],
            literals: &["true", "false", "TRUE", "FALSE", "NULL", "nothing", "Nothing", "nil", "#t", "#f"],
            ..Default::default()
        },
        _ => return None,
    };
    Some(s)
}

fn is_ident_start(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

fn is_ident(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

pub fn spans(lang: &str, text: &str) -> Vec<Span> {
    match lang {
        "HTML" | "XML" | "HTMLBars" | "Django" => markup(text),
        "CSS" | "SCSS" => css(text),
        "Diff" => diff(text),
        "Markdown" => markdown(text),
        _ => match syntax(lang) {
            Some(s) => code(&s, text),
            None => vec![],
        },
    }
}

fn push(out: &mut Vec<Span>, r: Range<usize>, t: Tok) {
    if r.start < r.end {
        out.push((r, color(t), t == Tok::Comment));
    }
}

/// End of a quoted string starting at `i` (handles backslash escapes;
/// unterminated strings run to the end of the line).
fn string_end(text: &str, i: usize, q: char) -> usize {
    let mut esc = false;
    for (k, ch) in text[i + q.len_utf8()..].char_indices() {
        let at = i + q.len_utf8() + k;
        if esc {
            esc = false;
        } else if ch == '\\' {
            esc = true;
        } else if ch == q {
            return at + ch.len_utf8();
        } else if ch == '\n' && q != '`' {
            return at;
        }
    }
    text.len()
}

fn number_end(text: &str, i: usize) -> usize {
    let b = text.as_bytes();
    let mut j = i;
    while j < b.len() && (b[j].is_ascii_alphanumeric() || b[j] == b'_' || (b[j] == b'.' && j + 1 < b.len() && b[j + 1].is_ascii_digit())) {
        j += 1;
    }
    j
}

fn code(s: &Syntax, text: &str) -> Vec<Span> {
    let mut out = vec![];
    let mut i = 0;
    let mut prev_word: Option<&str> = None;
    let has = |list: &[&str], w: &str| {
        if s.case_insensitive { list.iter().any(|k| k.eq_ignore_ascii_case(w)) } else { list.contains(&w) }
    };
    while i < text.len() {
        let rest = &text[i..];
        let ch = rest.chars().next().unwrap();
        if let Some(lc) = s.line_comments.iter().find(|lc| rest.starts_with(**lc)) {
            let _ = lc;
            let end = rest.find('\n').map_or(text.len(), |k| i + k);
            push(&mut out, i..end, Tok::Comment);
            i = end;
            continue;
        }
        if let Some((open, close)) = s.block_comment
            && rest.starts_with(open)
        {
            let end = rest[open.len()..].find(close).map_or(text.len(), |k| i + open.len() + k + close.len());
            push(&mut out, i..end, Tok::Comment);
            i = end;
            continue;
        }
        if s.py_strings && s.quotes.contains(ch) {
            let triple: String = std::iter::repeat_n(ch, 3).collect();
            if rest.starts_with(&triple) {
                let end = rest[3..].find(&triple).map_or(text.len(), |k| i + 3 + k + 3);
                push(&mut out, i..end, Tok::String);
                i = end;
                prev_word = None;
                continue;
            }
        }
        if s.quotes.contains(ch) {
            // Rust lifetimes ('a) are not strings.
            let lifetime = ch == '\'' && s.bang_macros && !rest[1..].chars().nth(1).is_some_and(|c| c == '\'') && !rest[1..].starts_with('\\');
            if !lifetime {
                let end = string_end(text, i, ch);
                push(&mut out, i..end, Tok::String);
                i = end;
                prev_word = None;
                continue;
            }
        }
        if !s.sigil_vars.is_empty() && s.sigil_vars.contains(ch) {
            let mut j = i + ch.len_utf8();
            if text[j..].starts_with('{') {
                j = text[j..].find('}').map_or(text.len(), |k| j + k + 1);
            } else {
                while let Some(c2) = text[j..].chars().next().filter(|c| is_ident(*c)) {
                    j += c2.len_utf8();
                }
            }
            if j > i + 1 {
                push(&mut out, i..j, Tok::Type);
                i = j;
                continue;
            }
        }
        if ch.is_ascii_digit() {
            let end = number_end(text, i);
            push(&mut out, i..end, Tok::Number);
            i = end;
            prev_word = None;
            continue;
        }
        if is_ident_start(ch) {
            let mut j = i;
            while let Some(c2) = text[j..].chars().next().filter(|c| is_ident(*c)) {
                j += c2.len_utf8();
            }
            let w = &text[i..j];
            let next = text[j..].chars().next();
            if s.py_strings
                && w.len() <= 2
                && w.chars().all(|c| "fFrRbBuU".contains(c))
                && let Some(q) = next.filter(|q| s.quotes.contains(*q))
            {
                let end = string_end(text, j, q);
                push(&mut out, i..end, Tok::String);
                i = end;
                prev_word = None;
                continue;
            }
            let tok = if prev_word.is_some_and(|p| has(s.defs, p)) {
                Some(Tok::Title)
            } else if s.bang_macros && next == Some('!') {
                j += 1;
                Some(Tok::BuiltIn)
            } else if has(s.keywords, w) {
                Some(Tok::Keyword)
            } else if has(s.literals, w) {
                Some(Tok::Literal)
            } else if has(s.types, w) {
                Some(Tok::Type)
            } else if has(s.builtins, w) {
                Some(Tok::BuiltIn)
            } else if s.caps_types && w.chars().next().is_some_and(|c| c.is_uppercase()) {
                Some(Tok::Type)
            } else {
                None
            };
            if let Some(t) = tok {
                push(&mut out, i..j, t);
            }
            prev_word = Some(w);
            i = j;
            continue;
        }
        if !ch.is_whitespace() {
            prev_word = None;
        }
        i += ch.len_utf8();
    }
    out
}

fn markup(text: &str) -> Vec<Span> {
    let mut out = vec![];
    let mut i = 0;
    while i < text.len() {
        let rest = &text[i..];
        if rest.starts_with("<!--") {
            let end = rest.find("-->").map_or(text.len(), |k| i + k + 3);
            push(&mut out, i..end, Tok::Comment);
            i = end;
            continue;
        }
        if rest.starts_with('<') {
            let mut j = i + 1;
            if text[j..].starts_with('/') || text[j..].starts_with('!') || text[j..].starts_with('?') {
                j += 1;
            }
            let name_start = j;
            while let Some(c2) = text[j..].chars().next().filter(|c| is_ident(*c) || *c == '-' || *c == ':' || *c == '.') {
                j += c2.len_utf8();
            }
            push(&mut out, name_start..j, Tok::Tag);
            // Attributes until the closing '>'.
            while j < text.len() && !text[j..].starts_with('>') {
                let c2 = text[j..].chars().next().unwrap();
                if c2 == '"' || c2 == '\'' {
                    let end = string_end(text, j, c2);
                    push(&mut out, j..end, Tok::String);
                    j = end;
                } else if is_ident_start(c2) {
                    let a = j;
                    while let Some(c3) = text[j..].chars().next().filter(|c| is_ident(*c) || *c == '-' || *c == ':') {
                        j += c3.len_utf8();
                    }
                    push(&mut out, a..j, Tok::Attr);
                } else {
                    j += c2.len_utf8();
                }
            }
            i = j;
            continue;
        }
        i += rest.chars().next().unwrap().len_utf8();
    }
    out
}

fn css(text: &str) -> Vec<Span> {
    let mut out = vec![];
    let mut i = 0;
    let mut depth = 0;
    while i < text.len() {
        let rest = &text[i..];
        let ch = rest.chars().next().unwrap();
        if let Some(body) = rest.strip_prefix("/*") {
            let end = body.find("*/").map_or(text.len(), |k| i + 2 + k + 2);
            push(&mut out, i..end, Tok::Comment);
            i = end;
            continue;
        }
        if rest.starts_with("//") {
            let end = rest.find('\n').map_or(text.len(), |k| i + k);
            push(&mut out, i..end, Tok::Comment);
            i = end;
            continue;
        }
        match ch {
            '{' => depth += 1,
            '}' => depth -= 1,
            '"' | '\'' => {
                let end = string_end(text, i, ch);
                push(&mut out, i..end, Tok::String);
                i = end;
                continue;
            }
            _ => {}
        }
        if ch.is_ascii_digit() || (ch == '#' && depth > 0) {
            let mut j = i + 1;
            while j < text.len() && (text.as_bytes()[j].is_ascii_alphanumeric() || text.as_bytes()[j] == b'.' || text.as_bytes()[j] == b'%') {
                j += 1;
            }
            push(&mut out, i..j, Tok::Number);
            i = j;
            continue;
        }
        if is_ident_start(ch) || ch == '-' || ch == '.' || ch == '#' || ch == '@' || ch == '$' {
            let mut j = i + ch.len_utf8();
            while let Some(c2) = text[j..].chars().next().filter(|c| is_ident(*c) || *c == '-') {
                j += c2.len_utf8();
            }
            let after = text[j..].trim_start_matches([' ', '\t']).chars().next();
            let tok = if ch == '@' {
                Tok::Keyword
            } else if ch == '$' {
                Tok::Type
            } else if depth > 0 && after == Some(':') {
                Tok::Attr
            } else if depth > 0 && after == Some('(') {
                // `url(…)` and friends colour like property names.
                Tok::Attr
            } else if depth == 0 {
                match ch {
                    '.' => Tok::Type,
                    '#' => Tok::Title,
                    _ => Tok::Tag,
                }
            } else {
                i = j;
                continue;
            };
            // The palette: property names in olive.
            let tok = if tok == Tok::Attr { Tok::BuiltIn } else { tok };
            push(&mut out, i..j, tok);
            i = j;
            continue;
        }
        i += ch.len_utf8();
    }
    out
}

fn diff(text: &str) -> Vec<Span> {
    let mut out = vec![];
    let mut at = 0;
    for line in text.split('\n') {
        let r = at..at + line.len();
        if line.starts_with('+') {
            push(&mut out, r, Tok::String);
        } else if line.starts_with('-') {
            push(&mut out, r, Tok::Tag);
        } else if line.starts_with("@@") {
            push(&mut out, r, Tok::Title);
        }
        at += line.len() + 1;
    }
    out
}

fn markdown(text: &str) -> Vec<Span> {
    let mut out = vec![];
    let mut at = 0;
    for line in text.split('\n') {
        let r = at..at + line.len();
        let t = line.trim_start();
        if t.starts_with('#') {
            push(&mut out, r, Tok::Tag);
        } else if t.starts_with("- ") || t.starts_with("* ") || t.starts_with("> ") {
            let lead = line.len() - t.len();
            push(&mut out, at + lead..at + lead + 1, Tok::Title);
        }
        at += line.len() + 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(lang: &str, text: &str) -> Vec<(String, u32)> {
        spans(lang, text)
            .into_iter()
            .map(|(r, h, _)| {
                let rgb: gpui::Rgba = h.into();
                let v = ((rgb.r * 255.).round() as u32) << 16 | ((rgb.g * 255.).round() as u32) << 8 | (rgb.b * 255.).round() as u32;
                (text[r].to_string(), v)
            })
            .collect()
    }

    #[test]
    fn rust() {
        let k = kinds("Rust", "fn main() {\n    println!(\"hi\"); // x\n    let n: u8 = 3;\n}");
        assert_eq!(
            k,
            vec![
                ("fn".into(), 0xb2565c),
                ("main".into(), 0x5f7e43),
                ("println!".into(), 0x5f7e43),
                ("\"hi\"".into(), 0x89713a),
                ("// x".into(), 0x8d9096),
                ("let".into(), 0xb2565c),
                ("u8".into(), 0x3878b1),
                ("3".into(), 0x9759b3),
            ]
        );
    }

    #[test]
    fn markup_tags() {
        let k = kinds("HTML", "<a href=\"x\">t</a>");
        assert_eq!(k[0], ("a".into(), 0xb2565c));
        assert_eq!(k[1], ("href".into(), 0x3878b1));
        assert_eq!(k[2], ("\"x\"".into(), 0x89713a));
        assert_eq!(k[3], ("a".into(), 0xb2565c));
    }

    #[test]
    fn python_strings() {
        let k = kinds("Python", "x = f\"a{b}\"\n\"\"\"l1\nl2\"\"\"");
        assert_eq!(k, vec![("f\"a{b}\"".into(), 0x89713a), ("\"\"\"l1\nl2\"\"\"".into(), 0x89713a)]);
    }

    #[test]
    fn plain_is_uncoloured() {
        assert!(spans("Plain Text", "fn main() {}").is_empty());
    }
}
