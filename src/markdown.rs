//! Transformă Markdown în HTML.
//!
//! Blocurile de cod Rust primesc un buton care deschide exemplul în
//! Rust Playground. Atributele de tip rustdoc schimbă afișarea:
//! `rust,compile_fail` marchează un exemplu care nu compilează intenționat,
//! iar `rust,ignore` ascunde butonul de rulare.

use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd, html};

const PLAYGROUND: &str = "https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=";

pub fn render(src: &str) -> String {
    let mut events = Vec::new();
    // Atributele blocului de cod Rust curent și codul strâns până acum.
    let mut rust_block: Option<(String, String)> = None;

    for event in Parser::new_ext(src, Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH) {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref lang))) if lang.starts_with("rust") => {
                rust_block = Some((lang.to_string(), String::new()));
            }
            Event::Text(text) if rust_block.is_some() => {
                rust_block.as_mut().unwrap().1.push_str(&text);
            }
            Event::End(TagEnd::CodeBlock) if rust_block.is_some() => {
                let (lang, code) = rust_block.take().unwrap();
                events.push(Event::Html(CowStr::from(code_block(&lang, &code))));
            }
            other => events.push(other),
        }
    }

    let mut out = String::new();
    html::push_html(&mut out, events.into_iter());
    out
}

fn code_block(lang: &str, code: &str) -> String {
    let fails = lang.contains("compile_fail");
    let mut header = String::new();
    if fails {
        header.push_str(r#"<span class="badge fail">Nu compilează (intenționat)</span>"#);
    }
    if !lang.contains("ignore") {
        header.push_str(&format!(
            r#"<a class="run" href="{PLAYGROUND}{}" target="_blank" rel="noopener">▶ Rulează</a>"#,
            percent_encode(code)
        ));
    }
    format!(
        r#"<div class="code{}"><div class="code-bar">{header}</div><pre><code>{}</code></pre></div>"#,
        if fails { " fails" } else { "" },
        escape_html(code)
    )
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
