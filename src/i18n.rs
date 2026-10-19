//! Textele interfeței, în fiecare limbă.

use crate::content::Lang;

pub struct Strings {
    pub home_title: &'static str,
    pub home_lead: &'static str,
    pub topics_heading: &'static str,
    pub search_placeholder: &'static str,
    pub search_label: &'static str,
    pub switch_label: &'static str,
    pub badge_compile_fail: &'static str,
    pub badge_should_panic: &'static str,
    pub run: &'static str,
    pub offline_heading: &'static str,
    pub offline_lead: &'static str,
}

const RO: Strings = Strings {
    home_title: "Rust, de la primul program până în compilator",
    home_lead: "Fiecare subiect are 4 niveluri. Toate nivelurile acoperă aceleași funcționalități ale limbajului; diferă doar cât de adânc intrăm în ele.",
    topics_heading: "Subiecte",
    search_placeholder: "Caută…  ( / )",
    search_label: "Caută în tot conținutul",
    switch_label: "Switch to English",
    badge_compile_fail: "Nu compilează (intenționat)",
    badge_should_panic: "Dă panic (intenționat)",
    run: "▶ Rulează",
    offline_heading: "Aplicație și offline",
    offline_lead: "Pe iPhone: Safari → Share → „Add to Home Screen”. Pe Android și desktop: meniul browserului → „Instalează aplicația”. După prima deschidere, toate lecțiile funcționează și fără internet.",
};

const EN: Strings = Strings {
    home_title: "Rust, from your first program to the compiler",
    home_lead: "Every topic has 4 levels. All levels cover the same language features; they differ only in how deep we go.",
    topics_heading: "Topics",
    search_placeholder: "Search…  ( / )",
    search_label: "Search all content",
    switch_label: "Treci la română",
    badge_compile_fail: "Does not compile (on purpose)",
    badge_should_panic: "Panics (on purpose)",
    run: "▶ Run",
    offline_heading: "App and offline",
    offline_lead: "On iPhone: Safari → Share → “Add to Home Screen”. On Android and desktop: the browser menu → “Install app”. After the first visit, every lesson works without internet too.",
};

pub fn strings(lang: Lang) -> &'static Strings {
    match lang {
        Lang::Ro => &RO,
        Lang::En => &EN,
    }
}

pub fn results_count(lang: Lang, n: usize, query: &str) -> String {
    match (lang, n) {
        (Lang::Ro, 0) => format!("Niciun rezultat pentru „{query}”"),
        (Lang::Ro, 1) => "1 rezultat".into(),
        // „20 de rezultate”, dar „19 rezultate”
        (Lang::Ro, n) if (1..20).contains(&(n % 100)) => format!("{n} rezultate"),
        (Lang::Ro, n) => format!("{n} de rezultate"),
        (Lang::En, 0) => format!("No results for “{query}”"),
        (Lang::En, 1) => "1 result".into(),
        (Lang::En, n) => format!("{n} results"),
    }
}
