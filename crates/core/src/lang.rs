//! Code block languages (behavior-spec §8.3). The document model stores the
//! display name; DocxXML's `lang` attribute uses a lowercase token.

/// (display name, XML token, extra aliases), in menu order.
pub const LANGS: &[(&str, &str, &[&str])] = &[
    ("Plain Text", "plaintext", &["plain", "text", "txt"]),
    ("ABAP", "abap", &[]),
    ("Ada", "ada", &[]),
    ("Apache", "apache", &[]),
    ("Apex", "apex", &[]),
    ("Assembly language", "assembly", &["asm"]),
    ("Bash", "bash", &["ash", "csh", "sh", "tcsh", "zsh"]),
    ("C#", "csharp", &["cs"]),
    ("C++", "cpp", &["cc", "hh"]),
    ("C", "c", &["h"]),
    ("CMake", "cmake", &[]),
    ("COBOL", "cobol", &[]),
    ("CSS", "css", &[]),
    ("CoffeeScript", "coffeescript", &["coffee"]),
    ("D", "d", &[]),
    ("Dart", "dart", &[]),
    ("Delphi", "delphi", &["dfm", "dpr", "pas", "pascal"]),
    ("Diff", "diff", &["patch"]),
    ("Django", "django", &["jinja"]),
    ("Dockerfile", "dockerfile", &["docker"]),
    ("Erlang", "erlang", &["erl"]),
    ("Fortran", "fortran", &["f90", "f95"]),
    ("Gherkin", "gherkin", &[]),
    ("Go", "go", &["golang"]),
    ("GraphQL", "graphql", &[]),
    ("Groovy", "groovy", &[]),
    ("HTML", "html", &[]),
    ("HTMLBars", "htmlbars", &[]),
    ("HTTP", "http", &["https"]),
    ("Haskell", "haskell", &["hs"]),
    ("JSON", "json", &[]),
    ("Java", "java", &["jsp"]),
    ("JavaScript", "javascript", &["js", "jsx"]),
    ("Julia", "julia", &[]),
    ("Kotlin", "kotlin", &["kt"]),
    ("LaTeX", "latex", &["math", "tex"]),
    ("Lisp", "lisp", &[]),
    ("Lua", "lua", &[]),
    ("MATLAB", "matlab", &[]),
    ("Makefile", "makefile", &["mak", "mk", "make"]),
    ("Markdown", "markdown", &["md"]),
    ("Nginx", "nginx", &[]),
    ("Objective-C", "objectivec", &["oc", "objc", "mm"]),
    ("OpenGL Shading Language", "glsl", &[]),
    ("PHP", "php", &[]),
    ("Perl", "perl", &["pl", "pm"]),
    ("PowerShell", "powershell", &["ps", "ps1"]),
    ("Prolog", "prolog", &[]),
    ("Properties", "properties", &[]),
    ("ProtoBuf", "protobuf", &["proto"]),
    ("Python", "python", &["py"]),
    ("R", "r", &[]),
    ("Ruby", "ruby", &["rb"]),
    ("Rust", "rust", &["rs"]),
    ("SAS", "sas", &[]),
    ("SCSS", "scss", &[]),
    ("SQL", "sql", &[]),
    ("Scala", "scala", &[]),
    ("Scheme", "scheme", &[]),
    ("Shell", "shell", &[]),
    ("Solidity", "solidity", &["sol"]),
    ("Swift", "swift", &[]),
    ("TOML", "toml", &["ini"]),
    ("Thrift", "thrift", &[]),
    ("TypeScript", "typescript", &["ts", "tsx"]),
    ("VBScript", "vbscript", &["vbs"]),
    ("VHDL", "vhdl", &[]),
    ("Verilog", "verilog", &["v"]),
    ("Visual Basic", "vb", &["visualbasic"]),
    ("XML", "xml", &["svg"]),
    ("YAML", "yaml", &["yml"]),
];

pub const PLAIN: &str = "Plain Text";

fn squash(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace() && *c != '-' && *c != '_').flat_map(char::to_lowercase).collect()
}

fn exact(s: &str) -> Option<&'static str> {
    let q = squash(s);
    LANGS
        .iter()
        .find(|(name, token, aliases)| squash(name) == q || *token == q || aliases.contains(&q.as_str()))
        .map(|(name, ..)| *name)
}

/// Display name for a DocxXML `lang` token. Unknown tokens are kept as-is
/// so they survive a round trip.
pub fn from_token(token: &str) -> String {
    if token.trim().is_empty() {
        return PLAIN.into();
    }
    exact(token).map_or_else(|| token.to_string(), str::to_string)
}

/// DocxXML `lang` token for a display name.
pub fn to_token(name: &str) -> String {
    LANGS.iter().find(|(n, ..)| *n == name).map_or_else(|| name.to_string(), |(_, t, _)| t.to_string())
}

/// Resolves a typed info string (``` fence, paste): exact alias, then display
/// name prefix, then fuzzy subsequence; Plain Text otherwise.
pub fn resolve(s: &str) -> &'static str {
    // A typed `shell` goes to Bash.
    if squash(s) == "shell" {
        return "Bash";
    }
    if let Some(n) = exact(s) {
        return n;
    }
    let q = squash(s);
    if q.is_empty() {
        return PLAIN;
    }
    if let Some((n, ..)) = LANGS.iter().find(|(n, ..)| squash(n).starts_with(&q)) {
        return n;
    }
    let subseq = |hay: &str| {
        let mut it = hay.chars();
        q.chars().all(|c| it.any(|h| h == c))
    };
    LANGS.iter().find(|(n, ..)| subseq(&squash(n))).map_or(PLAIN, |(n, ..)| n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens_round_trip() {
        for (name, token, _) in LANGS {
            assert_eq!(from_token(token), *name);
            assert_eq!(to_token(name), *token);
        }
        assert_eq!(from_token("brainfuck"), "brainfuck");
        assert_eq!(to_token("brainfuck"), "brainfuck");
    }

    #[test]
    fn resolve_info_strings() {
        assert_eq!(resolve(""), PLAIN);
        assert_eq!(resolve("rs"), "Rust");
        assert_eq!(resolve("zsh"), "Bash");
        assert_eq!(resolve("shell"), "Bash");
        assert_eq!(resolve("C++"), "C++");
        assert_eq!(resolve("type"), "TypeScript");
        assert_eq!(resolve("objective-c"), "Objective-C");
        assert_eq!(resolve("qqqq"), PLAIN);
    }
}
