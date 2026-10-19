//! Interfața aplicației.
//!
//! Navigarea folosește fragmentul din URL (`#/<slug>/<nivel>`, iar pentru
//! engleză `#/en/<slug>/<nivel>`), ca site-ul să funcționeze pe GitHub Pages
//! fără configurare de server.

use leptos::prelude::*;

use crate::content::{LEVELS, Lang, TOPICS};
use crate::i18n::{self, strings};
use crate::{markdown, search};

/// Căutarea pornește de la acest număr de caractere.
const MIN_QUERY: usize = 2;

#[derive(Clone, Copy, PartialEq)]
struct Route {
    topic: usize,
    /// Indexul nivelului, de la 0 la 3.
    level: usize,
}

/// Limba și pagina din fragmentul URL; pagina `None` înseamnă pagina de start.
fn parse_hash(hash: &str) -> (Lang, Option<Route>) {
    let path = hash.trim_start_matches('#').trim_start_matches('/');
    let (lang, path) = match path.strip_prefix("en") {
        Some(rest) if rest.is_empty() || rest.starts_with('/') => (Lang::En, rest.trim_start_matches('/')),
        _ => (Lang::Ro, path),
    };
    let mut parts = path.split('/');
    let route = parts.next().and_then(|slug| {
        let topic = TOPICS.iter().position(|t| t.slug == slug)?;
        let level = parts
            .next()
            .and_then(|l| l.parse::<usize>().ok())
            .filter(|l| (1..=LEVELS.len()).contains(l))
            .map_or(0, |l| l - 1);
        Some(Route { topic, level })
    });
    (lang, route)
}

fn prefix(lang: Lang) -> &'static str {
    match lang {
        Lang::Ro => "#/",
        Lang::En => "#/en/",
    }
}

fn href(lang: Lang, topic: usize, level: usize) -> String {
    format!("{}{}/{}", prefix(lang), TOPICS[topic].slug, level + 1)
}

fn current_hash() -> String {
    window().location().hash().unwrap_or_default()
}

#[component]
pub fn App() -> impl IntoView {
    let hash = RwSignal::new(current_hash());
    let query = RwSignal::new(String::new());
    let search_input = NodeRef::<leptos::html::Input>::new();

    let _ = window_event_listener(leptos::ev::hashchange, move |_| {
        hash.set(current_hash());
        query.set(String::new()); // orice navigare închide rezultatele
        window().scroll_to_with_x_and_y(0.0, 0.0);
    });
    // `/` mută cursorul în căutare, dacă nu scrii deja într-un câmp
    let _ = window_event_listener(leptos::ev::keydown, move |ev| {
        let typing = document()
            .active_element()
            .is_some_and(|el| matches!(el.tag_name().as_str(), "INPUT" | "TEXTAREA"));
        if ev.key() == "/" && !typing {
            ev.prevent_default();
            if let Some(input) = search_input.get() {
                let _ = input.focus();
            }
        }
    });

    let parsed = Memo::new(move |_| parse_hash(&hash.get()));
    let lang = Memo::new(move |_| parsed.get().0);
    let route = Memo::new(move |_| parsed.get().1);
    // Nivelul ales rămâne activ când treci la alt subiect.
    let level = Memo::new(move |_| route.get().map_or(0, |r| r.level));
    let searching = Memo::new(move |_| query.with(|q| q.trim().chars().count() >= MIN_QUERY));

    // <html lang> urmează limba paginii, pentru cititoarele de ecran și traducerea automată
    Effect::new(move |_| {
        if let Some(root) = document().document_element() {
            let _ = root.set_attribute("lang", lang.get().code());
        }
    });

    // Aceeași pagină, în cealaltă limbă
    let switch_href = move || {
        let other = lang.get().other();
        match route.get() {
            Some(r) => href(other, r.topic, r.level),
            None => prefix(other).to_string(),
        }
    };

    view! {
        <div class="layout">
            <aside class="sidebar">
                <div class="sidebar-top">
                    <a href=move || prefix(lang.get()) class="brand">"🦀 Rust Features"</a>
                    <a
                        href=switch_href
                        class="lang-toggle"
                        hreflang=move || lang.get().other().code()
                        title=move || strings(lang.get()).switch_label
                        aria-label=move || strings(lang.get()).switch_label
                    >
                        {move || lang.get().other().code().to_uppercase()}
                    </a>
                </div>
                <input
                    type="search"
                    class="search"
                    placeholder=move || strings(lang.get()).search_placeholder
                    aria-label=move || strings(lang.get()).search_label
                    node_ref=search_input
                    prop:value=move || query.get()
                    on:input=move |ev| query.set(event_target_value(&ev))
                    on:keydown=move |ev| {
                        if ev.key() == "Escape" {
                            query.set(String::new());
                        }
                    }
                />
                <nav>
                    {TOPICS
                        .iter()
                        .enumerate()
                        .map(|(i, t)| {
                            view! {
                                <a
                                    href=move || href(lang.get(), i, level.get())
                                    class:active=move || route.get().is_some_and(|r| r.topic == i)
                                >
                                    {move || t.title(lang.get())}
                                </a>
                            }
                        })
                        .collect_view()}
                </nav>
            </aside>
            <main>
                {move || {
                    let lang = lang.get();
                    if searching.get() {
                        view! { <SearchResults query=query lang=lang /> }.into_any()
                    } else {
                        match route.get() {
                            Some(r) => view! { <TopicPage route=r lang=lang /> }.into_any(),
                            None => view! { <Home lang=lang /> }.into_any(),
                        }
                    }
                }}
            </main>
        </div>
    }
}

#[component]
fn Home(lang: Lang) -> impl IntoView {
    let s = strings(lang);
    view! {
        <section class="home">
            <h1>{s.home_title}</h1>
            <p class="lead">{s.home_lead}</p>
            <div class="levels">
                {LEVELS
                    .iter()
                    .enumerate()
                    .map(|(i, l)| {
                        view! {
                            <div class="level-card">
                                <span class="level-num">{i + 1}</span>
                                <h3>{l.name(lang)}</h3>
                                <p>{l.tagline(lang)}</p>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
            <h2>{s.topics_heading}</h2>
            <div class="topics">
                {TOPICS
                    .iter()
                    .enumerate()
                    .map(|(i, t)| view! { <a class="topic-card" href=href(lang, i, 0)>{t.title(lang)}</a> })
                    .collect_view()}
            </div>
            <h2>{s.offline_heading}</h2>
            <div class="offline-card">
                <p>{s.offline_lead}</p>
            </div>
        </section>
    }
}

#[component]
fn SearchResults(query: RwSignal<String>, lang: Lang) -> impl IntoView {
    let hits = Memo::new(move |_| query.with(|q| search::search(q.trim(), lang)));

    view! {
        <section class="results">
            <p class="results-count" aria-live="polite">
                {move || i18n::results_count(lang, hits.with(Vec::len), query.get().trim())}
            </p>
            {move || {
                hits.with(|hits| {
                    hits.iter()
                        .map(|h| {
                            let (before, matched, after) = h.snippet.clone();
                            view! {
                                <a class="result" href=href(lang, h.topic, h.level)>
                                    <span class="result-title">
                                        {TOPICS[h.topic].title(lang)}
                                        <span class="result-level">
                                            <span class="level-num">{h.level + 1}</span>
                                            {LEVELS[h.level].name(lang)}
                                        </span>
                                    </span>
                                    <span class="result-snippet">
                                        {before}
                                        <mark>{matched}</mark>
                                        {after}
                                    </span>
                                </a>
                            }
                        })
                        .collect_view()
                })
            }}
        </section>
    }
}

#[component]
fn TopicPage(route: Route, lang: Lang) -> impl IntoView {
    let topic = &TOPICS[route.topic];
    let prev = route.topic.checked_sub(1);
    let next = Some(route.topic + 1).filter(|&n| n < TOPICS.len());

    view! {
        <div class="level-tabs" role="tablist">
            {LEVELS
                .iter()
                .enumerate()
                .map(|(i, l)| {
                    view! {
                        <a
                            href=href(lang, route.topic, i)
                            class="level-tab"
                            class:active=i == route.level
                            role="tab"
                            aria-selected=(i == route.level).to_string()
                        >
                            <span class="level-num">{i + 1}</span>
                            {l.name(lang)}
                        </a>
                    }
                })
                .collect_view()}
        </div>
        <article class="content" inner_html=markdown::render(topic.page(lang, route.level), lang) />
        <nav class="pager">
            {prev.map(|p| view! { <a href=href(lang, p, route.level)>"← " {TOPICS[p].title(lang)}</a> })}
            <span />
            {next.map(|n| view! { <a href=href(lang, n, route.level)>{TOPICS[n].title(lang)} " →"</a> })}
        </nav>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn romanian_urls_keep_working() {
        assert_eq!(parse_hash("#/ownership/2"), (Lang::Ro, Some(Route { topic: 3, level: 1 })));
        assert_eq!(parse_hash(""), (Lang::Ro, None));
        assert_eq!(parse_hash("#/"), (Lang::Ro, None));
    }

    #[test]
    fn english_urls() {
        assert_eq!(parse_hash("#/en/ownership/4"), (Lang::En, Some(Route { topic: 3, level: 3 })));
        assert_eq!(parse_hash("#/en"), (Lang::En, None));
        assert_eq!(parse_hash("#/en/"), (Lang::En, None));
        assert_eq!(href(Lang::En, 3, 0), "#/en/ownership/1");
    }

    #[test]
    fn slug_starting_with_en_is_not_english() {
        // un subiect al cărui slug ar începe cu „en” nu trebuie confundat cu prefixul de limbă
        assert_eq!(parse_hash("#/enums/1").0, Lang::Ro);
    }

    impl std::fmt::Debug for Route {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Route({}, {})", self.topic, self.level)
        }
    }
}
