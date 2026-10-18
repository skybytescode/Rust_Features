//! Colorarea sintaxei pentru blocurile de cod, generată ca HTML cu clase CSS.
//!
//! Pentru Rust e un tokenizer simplu, nu un parser complet: recunoaște
//! comentarii, string-uri, caractere, lifetimes, numere, cuvinte cheie,
//! tipuri, macro-uri, atribute și apeluri de funcții. Pentru TOML și shell
//! colorează doar comentariile, string-urile și secțiunile.

const KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "safe", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "union", "unsafe", "use", "where", "while", "yield",
];

const PRIMITIVES: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32",
    "f64", "bool", "char", "str",
];

pub fn rust(code: &str) -> String {
    let c: Vec<char> = code.chars().collect();
    let mut out = String::with_capacity(code.len() * 2);
    let mut i = 0;

    while i < c.len() {
        let start = i;
        let ch = c[i];
        let next = c.get(i + 1).copied();

        let class = if ch == '/' && next == Some('/') {
            i = find(&c, i, |c, j| c[j] == '\n');
            "com"
        } else if ch == '/' && next == Some('*') {
            i = (i + 2..c.len().saturating_sub(1))
                .find(|&j| c[j] == '*' && c[j + 1] == '/')
                .map_or(c.len(), |j| j + 2);
            "com"
        } else if ch == '#' && (next == Some('[') || (next == Some('!') && c.get(i + 2) == Some(&'['))) {
            i = attribute_end(&c, i);
            "attr"
        } else if let Some(end) = string_end(&c, i) {
            i = end;
            "str"
        } else if ch == '\'' {
            match char_literal_end(&c, i) {
                Some(end) => {
                    i = end;
                    "str"
                }
                None => {
                    i = find(&c, i + 1, |c, j| !is_ident(c[j]));
                    "lt"
                }
            }
        } else if ch.is_ascii_digit() {
            i = number_end(&c, i);
            "num"
        } else if ch.is_alphabetic() || ch == '_' {
            i = find(&c, i, |c, j| !is_ident(c[j]));
            let word: String = c[start..i].iter().collect();
            if c.get(i) == Some(&'!') && c.get(i + 1) != Some(&'=') {
                i += 1;
                "mac"
            } else if KEYWORDS.contains(&word.as_str()) {
                "kw"
            } else if PRIMITIVES.contains(&word.as_str()) || word.starts_with(char::is_uppercase) {
                "ty"
            } else if c.get(i) == Some(&'(') || c[i..].starts_with(&[':', ':', '<']) {
                "fn"
            } else {
                ""
            }
        } else {
            i += 1;
            ""
        };

        push_token(&mut out, class, &c[start..i]);
    }
    out
}

pub fn simple(code: &str, lang: &str) -> String {
    let mut out = String::with_capacity(code.len() * 2);
    for line in code.split_inclusive('\n') {
        let c: Vec<char> = line.chars().collect();
        let trimmed = line.trim_start();
        if lang == "toml" && trimmed.starts_with('[') {
            let indent = line.len() - trimmed.len();
            push_token(&mut out, "", &c[..indent]);
            let body: Vec<char> = trimmed.trim_end().chars().collect();
            push_token(&mut out, "kw", &body);
            push_token(&mut out, "", &c[indent + body.len()..]);
            continue;
        }
        let mut i = 0;
        while i < c.len() {
            let start = i;
            let class = if c[i] == '#' && (i == 0 || c[i - 1].is_whitespace()) {
                i = find(&c, i, |c, j| c[j] == '\n');
                "com"
            } else if c[i] == '"' || c[i] == '\'' {
                let quote = c[i];
                i = (i + 1..c.len()).find(|&j| c[j] == quote && c[j - 1] != '\\').map_or(c.len(), |j| j + 1);
                "str"
            } else if lang == "toml" && c[i].is_ascii_digit() && (i == 0 || !is_ident(c[i - 1])) {
                i = find(&c, i, |c, j| !(c[j].is_ascii_alphanumeric() || c[j] == '.' || c[j] == '_'));
                "num"
            } else if lang != "toml" && i == leading_ws(&c) && c[i].is_alphabetic() {
                // primul cuvânt de pe linie: comanda
                i = find(&c, i, |c, j| c[j].is_whitespace());
                "fn"
            } else {
                i += 1;
                ""
            };
            push_token(&mut out, class, &c[start..i]);
        }
    }
    out
}

fn leading_ws(c: &[char]) -> usize {
    c.iter().take_while(|ch| **ch == ' ' || **ch == '\t').count()
}

fn is_ident(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

/// Primul index `j >= from` pentru care `stop` e adevărat, sau lungimea.
fn find(c: &[char], from: usize, stop: impl Fn(&[char], usize) -> bool) -> usize {
    (from..c.len()).find(|&j| stop(c, j)).unwrap_or(c.len())
}

fn attribute_end(c: &[char], start: usize) -> usize {
    let mut depth = 0;
    for j in start..c.len() {
        match c[j] {
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth == 0 {
                    return j + 1;
                }
            }
            _ => {}
        }
    }
    c.len()
}

/// Recunoaște `"..."`, `b"..."`, `c"..."`, `r#"..."#`, `br"..."` și `cr"..."`.
fn string_end(c: &[char], start: usize) -> Option<usize> {
    let mut i = start;
    if i > 0 && is_ident(c[i - 1]) {
        return None;
    }
    if matches!(c.get(i), Some('b' | 'c')) {
        i += 1;
    }
    let raw = c.get(i) == Some(&'r');
    if raw {
        i += 1;
        let hashes = c[i..].iter().take_while(|&&ch| ch == '#').count();
        i += hashes;
        if c.get(i) != Some(&'"') {
            return None;
        }
        let closing: Vec<char> = std::iter::once('"').chain(std::iter::repeat_n('#', hashes)).collect();
        return Some((i + 1..c.len()).find(|&j| c[j..].starts_with(&closing)).map_or(c.len(), |j| j + closing.len()));
    }
    if c.get(i) != Some(&'"') {
        return None;
    }
    i += 1;
    while i < c.len() {
        match c[i] {
            '\\' => i += 2,
            '"' => return Some(i + 1),
            _ => i += 1,
        }
    }
    Some(c.len())
}

/// `'a'`, `'\n'`, `'\u{1F980}'` sunt caractere; `'a` fără apostrof final e un lifetime.
fn char_literal_end(c: &[char], start: usize) -> Option<usize> {
    match c.get(start + 1)? {
        '\\' => (start + 2..c.len()).find(|&j| c[j] == '\'').map(|j| j + 1),
        _ if c.get(start + 2) == Some(&'\'') => Some(start + 3),
        _ => None,
    }
}

fn number_end(c: &[char], start: usize) -> usize {
    let mut i = start;
    while i < c.len() {
        let ch = c[i];
        let decimal_point = ch == '.' && c.get(i + 1).is_some_and(|n| n.is_ascii_digit());
        if ch.is_ascii_alphanumeric() || ch == '_' || decimal_point {
            i += 1;
        } else {
            break;
        }
    }
    i
}

fn push_token(out: &mut String, class: &str, text: &[char]) {
    if text.is_empty() {
        return;
    }
    if !class.is_empty() {
        out.push_str(&format!(r#"<span class="tok-{class}">"#));
    }
    for &ch in text {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(ch),
        }
    }
    if !class.is_empty() {
        out.push_str("</span>");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keywords_types_and_macros() {
        let h = rust("let v: Vec<u8> = vec![1];");
        assert!(h.contains(r#"<span class="tok-kw">let</span>"#));
        assert!(h.contains(r#"<span class="tok-ty">Vec</span>&lt;<span class="tok-ty">u8</span>&gt;"#));
        assert!(h.contains(r#"<span class="tok-mac">vec!</span>"#));
        assert!(h.contains(r#"<span class="tok-num">1</span>"#));
    }

    #[test]
    fn lifetimes_vs_chars() {
        let h = rust("fn f<'a>(x: &'a str) -> char { 'x' }");
        assert!(h.contains(r#"<span class="tok-lt">'a</span>"#));
        assert!(h.contains(r#"<span class="tok-str">'x'</span>"#));
    }

    #[test]
    fn strings_comments_and_attributes() {
        let h = rust("#[derive(Debug)]\n// a < b\nlet s = r#\"x\"#; let t = \"a\\\"b\";");
        assert!(h.contains(r#"<span class="tok-attr">#[derive(Debug)]</span>"#));
        assert!(h.contains(r#"<span class="tok-com">// a &lt; b</span>"#));
        assert!(h.contains(r##"<span class="tok-str">r#&quot;x&quot;#</span>"##));
        assert!(h.contains(r#"<span class="tok-str">&quot;a\&quot;b&quot;</span>"#));
    }

    #[test]
    fn ranges_are_not_floats() {
        let h = rust("0..5");
        assert!(h.contains(r#"<span class="tok-num">0</span>..<span class="tok-num">5</span>"#));
    }

    #[test]
    fn not_equal_is_not_a_macro() {
        let h = rust("a != b");
        assert!(!h.contains("tok-mac"));
    }

    #[test]
    fn highlighting_preserves_text() {
        let src = "fn main() {\n    let x = 'a';\n    println!(\"{x} & <y>\");\n}\n";
        let stripped = strip_tags(&rust(src));
        assert_eq!(stripped, "fn main() {\n    let x = 'a';\n    println!(&quot;{x} &amp; &lt;y&gt;&quot;);\n}\n");
    }

    #[test]
    fn toml_sections_and_comments() {
        let h = simple("[package]\nname = \"x\" # nume\n", "toml");
        assert!(h.contains(r#"<span class="tok-kw">[package]</span>"#));
        assert!(h.contains(r#"<span class="tok-str">&quot;x&quot;</span>"#));
        assert!(h.contains(r#"<span class="tok-com"># nume</span>"#));
    }

    #[test]
    fn all_site_code_blocks_keep_their_text() {
        let mut blocks = 0;
        for topic in crate::content::TOPICS {
            for page in topic.levels {
                let mut lines = page.lines();
                while let Some(line) = lines.next() {
                    let fence: String = line.chars().take_while(|&c| c == '`').collect();
                    if fence.len() < 3 {
                        continue;
                    }
                    let lang = &line[fence.len()..];
                    let mut code = String::new();
                    for l in lines.by_ref() {
                        if l == fence {
                            break;
                        }
                        code.push_str(l);
                        code.push('\n');
                    }
                    let h = if lang.starts_with("rust") { rust(&code) } else { simple(&code, lang) };
                    assert_eq!(strip_tags(&h), crate::markdown::escape_for_test(&code), "{} / {lang}", topic.slug);
                    blocks += 1;
                }
            }
        }
        assert!(blocks > 400, "doar {blocks} blocuri găsite");
    }

    fn strip_tags(s: &str) -> String {
        let mut out = String::new();
        let mut in_tag = false;
        for ch in s.chars() {
            match ch {
                '<' => in_tag = true,
                '>' => in_tag = false,
                _ if !in_tag => out.push(ch),
                _ => {}
            }
        }
        out
    }
}
