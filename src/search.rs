//! Căutare în tot conținutul, direct în browser.
//!
//! Indexul se construiește la prima căutare, din Markdown-ul inclus în binar.
//! Textul e normalizat caracter cu caracter (litere mici, fără diacritice),
//! așa că pozițiile din textul normalizat corespund celor din original și
//! fragmentul afișat poate fi decupat direct.

use std::sync::LazyLock;

use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

use crate::content::TOPICS;

const SNIPPET_RADIUS: usize = 70;
const MAX_RESULTS: usize = 30;

struct Entry {
    topic: usize,
    level: usize,
    title: Vec<char>,
    headings: Vec<Vec<char>>,
    text: Vec<char>,
    folded_text: Vec<char>,
}

#[derive(PartialEq)]
pub struct Hit {
    pub topic: usize,
    pub level: usize,
    /// Fragmentul din jurul primei potriviri: (înainte, potrivire, după).
    pub snippet: (String, String, String),
}

static INDEX: LazyLock<Vec<Entry>> = LazyLock::new(build_index);

fn build_index() -> Vec<Entry> {
    let mut entries = Vec::new();
    for (topic, t) in TOPICS.iter().enumerate() {
        for (level, page) in t.levels.iter().enumerate() {
            let (headings, text) = plain_text(page);
            entries.push(Entry {
                topic,
                level,
                title: fold(t.title),
                headings: headings.iter().map(|h| fold(h)).collect(),
                folded_text: fold(&text),
                text: text.chars().collect(),
            });
        }
    }
    entries
}

/// Extrage subtitlurile și textul simplu (inclusiv codul) dintr-o pagină Markdown.
/// Titlul principal (`#`) nu intră în text: repetă doar numele subiectului și nivelul.
fn plain_text(markdown: &str) -> (Vec<String>, String) {
    let mut headings = Vec::new();
    let mut text = String::new();
    let mut heading: Option<String> = None;
    let mut in_title = false;
    for event in Parser::new(markdown) {
        match event {
            Event::Start(Tag::Heading { level: HeadingLevel::H1, .. }) => in_title = true,
            Event::End(TagEnd::Heading(HeadingLevel::H1)) => in_title = false,
            _ if in_title => {}
            Event::Start(Tag::Heading { .. }) => heading = Some(String::new()),
            Event::End(TagEnd::Heading(_)) => {
                headings.extend(heading.take());
                text.push(' ');
            }
            Event::Text(t) | Event::Code(t) => {
                if let Some(h) = heading.as_mut() {
                    h.push_str(&t);
                }
                text.push_str(&t);
            }
            Event::SoftBreak
            | Event::HardBreak
            | Event::End(TagEnd::Paragraph | TagEnd::Item | TagEnd::CodeBlock | TagEnd::TableCell) => {
                text.push(' ');
            }
            _ => {}
        }
    }
    (headings, text)
}

/// Litere mici și fără diacritice, păstrând un caracter pentru fiecare caracter.
fn fold(s: &str) -> Vec<char> {
    s.chars().map(fold_char).collect()
}

fn fold_char(c: char) -> char {
    match c {
        'ă' | 'â' | 'Ă' | 'Â' | 'á' | 'à' | 'ä' => 'a',
        'î' | 'Î' | 'í' => 'i',
        'ș' | 'ş' | 'Ș' | 'Ş' => 's',
        'ț' | 'ţ' | 'Ț' | 'Ţ' => 't',
        'é' | 'è' | 'ë' => 'e',
        'ó' | 'ö' => 'o',
        'ú' | 'ü' => 'u',
        _ => c.to_lowercase().next().unwrap_or(c),
    }
}

fn find(haystack: &[char], needle: &[char]) -> Option<usize> {
    if needle.is_empty() || needle.len() > haystack.len() {
        return None;
    }
    haystack.windows(needle.len()).position(|w| w == needle)
}

fn count(haystack: &[char], needle: &[char]) -> usize {
    if needle.is_empty() || needle.len() > haystack.len() {
        return 0;
    }
    haystack.windows(needle.len()).filter(|w| *w == needle).count()
}

pub fn search(query: &str) -> Vec<Hit> {
    let terms: Vec<Vec<char>> = query.split_whitespace().map(fold).collect();
    if terms.is_empty() {
        return Vec::new();
    }

    // (câți termeni apar în titlu, scor din conținut, intrare)
    let mut scored: Vec<(usize, usize, &Entry)> = INDEX
        .iter()
        .filter_map(|e| {
            let (mut title_terms, mut score) = (0, 0);
            for term in &terms {
                let in_title = find(&e.title, term).is_some();
                let in_headings = e.headings.iter().filter(|h| find(h, term).is_some()).count();
                let in_text = count(&e.folded_text, term);
                if !in_title && in_headings == 0 && in_text == 0 {
                    return None; // toate cuvintele trebuie să apară
                }
                title_terms += usize::from(in_title);
                score += in_headings.min(3) * 5 + in_text.min(10);
            }
            Some((title_terms, score, e))
        })
        .collect();

    // Întâi subiectele al căror titlu se potrivește, în ordinea nivelurilor;
    // apoi restul, după scor; la egalitate, ordinea din cuprins.
    scored.sort_by(|a, b| {
        let content_order = (a.2.topic, a.2.level).cmp(&(b.2.topic, b.2.level));
        b.0.cmp(&a.0).then(if a.0 > 0 { content_order } else { b.1.cmp(&a.1).then(content_order) })
    });

    scored
        .into_iter()
        .take(MAX_RESULTS)
        .map(|(_, _, e)| Hit { topic: e.topic, level: e.level, snippet: snippet(e, &terms) })
        .collect()
}

fn snippet(e: &Entry, terms: &[Vec<char>]) -> (String, String, String) {
    // primul termen care apare în text; altfel, începutul paginii
    let Some((pos, len)) = terms.iter().find_map(|t| find(&e.folded_text, t).map(|p| (p, t.len()))) else {
        let start: String = e.text.iter().take(SNIPPET_RADIUS * 2).collect();
        return (start.trim().to_string(), String::new(), "…".into());
    };

    let mut from = pos.saturating_sub(SNIPPET_RADIUS);
    let mut to = (pos + len + SNIPPET_RADIUS).min(e.text.len());
    // nu tăia cuvintele la margini
    while from > 0 && !e.text[from - 1].is_whitespace() {
        from += 1;
        if from >= pos {
            from = pos;
            break;
        }
    }
    while to < e.text.len() && !e.text[to].is_whitespace() && to > pos + len {
        to -= 1;
    }

    let collapse = |s: &[char]| s.iter().collect::<String>().split_whitespace().collect::<Vec<_>>().join(" ");
    let mut before = collapse(&e.text[from..pos]);
    let matched: String = e.text[pos..pos + len].iter().collect();
    let mut after = collapse(&e.text[pos + len..to]);
    if from > 0 {
        before.insert(0, '…');
    }
    if to < e.text.len() {
        after.push('…');
    }
    // păstrează spațiile din jurul potrivirii, eliminate de `collapse`
    if e.text[..pos].last().is_some_and(|c| c.is_whitespace()) && !before.is_empty() {
        before.push(' ');
    }
    if e.text.get(pos + len).is_some_and(|c| c.is_whitespace()) && !after.is_empty() {
        after.insert(0, ' ');
    }
    (before, matched, after)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slug(h: &Hit) -> &'static str {
        TOPICS[h.topic].slug
    }

    #[test]
    fn diacritics_are_ignored() {
        let fara = search("functii");
        let cu = search("Funcții");
        assert!(!fara.is_empty());
        assert_eq!(slug(&fara[0]), "functii");
        assert_eq!(fara.len(), cu.len());
    }

    #[test]
    fn title_matches_rank_first_in_level_order() {
        let hits = search("lifetimes");
        let primele: Vec<_> = hits.iter().take(4).map(|h| (slug(h), h.level)).collect();
        assert_eq!(primele, [("lifetimes", 0), ("lifetimes", 1), ("lifetimes", 2), ("lifetimes", 3)]);
    }

    #[test]
    fn snippet_skips_page_title() {
        // „Concurență — Fundamente” e titlul paginii; fragmentul trebuie să vină din text
        let hit = search("concurenta").into_iter().next().unwrap();
        let tot = format!("{}{}{}", hit.snippet.0, hit.snippet.1, hit.snippet.2);
        assert!(!tot.contains("— Fundamente"), "{tot}");
    }

    #[test]
    fn all_terms_must_match() {
        for h in search("refcell panic") {
            let e = INDEX.iter().find(|e| e.topic == h.topic && e.level == h.level).unwrap();
            assert!(find(&e.folded_text, &fold("refcell")).is_some());
            assert!(find(&e.folded_text, &fold("panic")).is_some());
        }
        assert!(search("cuvantcarenuexista").is_empty());
    }

    #[test]
    fn snippet_contains_original_match() {
        let hits = search("concurenta");
        let (_, matched, _) = &hits.iter().find(|h| !h.snippet.1.is_empty()).unwrap().snippet;
        assert_eq!(fold(matched), fold("concurenta"));
    }

    #[test]
    fn code_is_searchable() {
        assert!(search("split_at_mut").iter().any(|h| slug(h) == "unsafe-ffi"));
    }

    #[test]
    fn fold_keeps_length() {
        let s = "Ștefan și Țara Românească — İ";
        assert_eq!(fold(s).len(), s.chars().count());
    }
}
