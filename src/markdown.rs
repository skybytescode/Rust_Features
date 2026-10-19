//! Transformă Markdown în HTML.
//!
//! Blocurile de cod Rust primesc un buton care deschide exemplul în
//! Rust Playground. Atributele de tip rustdoc schimbă afișarea:
//! `rust,compile_fail` marchează un exemplu care nu compilează intenționat,
//! `rust,should_panic` unul care dă panic intenționat, iar `rust,ignore`
//! ascunde butonul de rulare.
//!
//! Toate blocurile de cod sunt colorate de `crate::highlight`.

use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd, html};

use crate::content::Lang;
use crate::i18n::strings;

const PLAYGROUND: &str = "https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=";

pub fn render(src: &str, lang: Lang) -> String {
    let mut events = Vec::new();
    // Atributele blocului de cod curent (limbajul etc.) și codul strâns până acum.
    let mut block: Option<(String, String)> = None;

    for event in Parser::new_ext(src, Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH) {
        match event {
            Event::Start(Tag::CodeBlock(ref kind)) => {
                let lang = match kind {
                    CodeBlockKind::Fenced(lang) => lang.to_string(),
                    CodeBlockKind::Indented => String::new(),
                };
                block = Some((lang, String::new()));
            }
            Event::Text(text) if block.is_some() => {
                block.as_mut().unwrap().1.push_str(&text);
            }
            Event::End(TagEnd::CodeBlock) if block.is_some() => {
                let (info, code) = block.take().unwrap();
                let html = if info.starts_with("rust") {
                    code_block(&info, &code, lang)
                } else {
                    plain_block(&info, &code)
                };
                events.push(Event::Html(CowStr::from(html)));
            }
            other => events.push(other),
        }
    }

    let mut out = String::new();
    html::push_html(&mut out, events.into_iter());
    out
}

fn code_block(info: &str, code: &str, lang: Lang) -> String {
    let s = strings(lang);
    let fails = info.contains("compile_fail");
    let mut header = String::new();
    if fails {
        header.push_str(&format!(r#"<span class="badge fail">{}</span>"#, s.badge_compile_fail));
    } else if info.contains("should_panic") {
        header.push_str(&format!(r#"<span class="badge fail">{}</span>"#, s.badge_should_panic));
    }
    if !info.contains("ignore") {
        header.push_str(&format!(
            r#"<a class="run" href="{PLAYGROUND}{}" target="_blank" rel="noopener">{}</a>"#,
            percent_encode(code),
            s.run
        ));
    }
    format!(
        r#"<div class="code{}"><div class="code-bar">{header}</div><pre><code>{}</code></pre></div>"#,
        if fails { " fails" } else { "" },
        crate::highlight::rust(code)
    )
}

fn plain_block(lang: &str, code: &str) -> String {
    let body = match lang {
        "toml" | "bash" | "sh" => crate::highlight::simple(code, lang),
        _ => escape_html(code),
    };
    format!("<pre><code>{body}</code></pre>")
}

#[cfg(test)]
pub fn escape_for_test(s: &str) -> String {
    escape_html(s)
}

fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
    out
}

fn percent_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}
