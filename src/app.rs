//! Interfața aplicației.
//!
//! Navigarea folosește fragmentul din URL (`#/<slug>/<nivel>`), ca site-ul
//! să funcționeze pe GitHub Pages fără configurare de server.

use leptos::prelude::*;

use crate::content::{LEVELS, TOPICS};
use crate::{markdown, search};

/// Căutarea pornește de la acest număr de caractere.
const MIN_QUERY: usize = 2;

#[derive(Clone, Copy, PartialEq)]
struct Route {
    topic: usize,
    /// Indexul nivelului, de la 0 la 3.
    level: usize,
}

/// `None` înseamnă pagina de start.
fn parse_hash(hash: &str) -> Option<Route> {
    let mut parts = hash.trim_start_matches('#').trim_start_matches('/').split('/');
    let slug = parts.next()?;
    let topic = TOPICS.iter().position(|t| t.slug == slug)?;
    let level = parts
        .next()
        .and_then(|l| l.parse::<usize>().ok())
        .filter(|l| (1..=LEVELS.len()).contains(l))
        .map_or(0, |l| l - 1);
    Some(Route { topic, level })
}

fn href(topic: usize, level: usize) -> String {
    format!("#/{}/{}", TOPICS[topic].slug, level + 1)
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

    let route = Memo::new(move |_| parse_hash(&hash.get()));
    // Nivelul ales rămâne activ când treci la alt subiect.
    let level = Memo::new(move |_| route.get().map_or(0, |r| r.level));
    let searching = Memo::new(move |_| query.with(|q| q.trim().chars().count() >= MIN_QUERY));

    view! {
        <div class="layout">
            <aside class="sidebar">
                <a href="#/" class="brand">"🦀 Rust Futures"</a>
                <input
                    type="search"
                    class="search"
                    placeholder="Caută…  ( / )"
                    aria-label="Caută în tot conținutul"
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
                                    href=move || href(i, level.get())
                                    class:active=move || route.get().is_some_and(|r| r.topic == i)
                                >
                                    {t.title}
                                </a>
                            }
                        })
                        .collect_view()}
                </nav>
            </aside>
            <main>
                {move || {
                    if searching.get() {
                        view! { <SearchResults query=query /> }.into_any()
                    } else {
                        match route.get() {
                            Some(r) => view! { <TopicPage route=r /> }.into_any(),
                            None => view! { <Home /> }.into_any(),
                        }
                    }
                }}
            </main>
        </div>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <section class="home">
            <h1>"Rust, de la primul program până în compilator"</h1>
            <p class="lead">
                "Fiecare subiect are 4 niveluri. Toate nivelurile acoperă aceleași funcționalități ale limbajului; diferă doar cât de adânc intrăm în ele."
            </p>
            <div class="levels">
                {LEVELS
                    .iter()
                    .enumerate()
                    .map(|(i, l)| {
                        view! {
                            <div class="level-card">
                                <span class="level-num">{i + 1}</span>
                                <h3>{l.name}</h3>
                                <p>{l.tagline}</p>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
            <h2>"Subiecte"</h2>
            <div class="topics">
                {TOPICS
                    .iter()
                    .enumerate()
                    .map(|(i, t)| view! { <a class="topic-card" href=href(i, 0)>{t.title}</a> })
                    .collect_view()}
            </div>
        </section>
    }
}

#[component]
fn SearchResults(query: RwSignal<String>) -> impl IntoView {
    let hits = Memo::new(move |_| query.with(|q| search::search(q.trim())));

    view! {
        <section class="results">
            <p class="results-count" aria-live="polite">
                {move || match hits.with(Vec::len) {
                    0 => format!("Niciun rezultat pentru „{}”", query.get().trim()),
                    1 => "1 rezultat".to_string(),
                    n => format!("{n} rezultate"),
                }}
            </p>
            {move || {
                hits.with(|hits| {
                    hits.iter()
                        .map(|h| {
                            let (before, matched, after) = h.snippet.clone();
                            view! {
                                <a class="result" href=href(h.topic, h.level)>
                                    <span class="result-title">
                                        {TOPICS[h.topic].title}
                                        <span class="result-level">
                                            <span class="level-num">{h.level + 1}</span>
                                            {LEVELS[h.level].name}
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
fn TopicPage(route: Route) -> impl IntoView {
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
                            href=href(route.topic, i)
                            class="level-tab"
                            class:active=i == route.level
                            role="tab"
                            aria-selected=(i == route.level).to_string()
                        >
                            <span class="level-num">{i + 1}</span>
                            {l.name}
                        </a>
                    }
                })
                .collect_view()}
        </div>
        <article class="content" inner_html=markdown::render(topic.levels[route.level]) />
        <nav class="pager">
            {prev.map(|p| view! { <a href=href(p, route.level)>"← " {TOPICS[p].title}</a> })}
            <span />
            {next.map(|n| view! { <a href=href(n, route.level)>{TOPICS[n].title} " →"</a> })}
        </nav>
    }
}
