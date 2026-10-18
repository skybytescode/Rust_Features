//! Catalogul de conținut: nivelurile și subiectele aplicației.
//!
//! Fiecare subiect are câte un fișier Markdown pentru fiecare nivel, în
//! `content/<slug>/<nivel>.md`. Fișierele sunt incluse în binar la compilare,
//! deci un fișier lipsă e o eroare de compilare, nu o pagină goală.

pub struct Level {
    pub name: &'static str,
    pub tagline: &'static str,
}

pub const LEVELS: [Level; 4] = [
    Level { name: "Fundamente", tagline: "Ce este și cum îl folosesc" },
    Level { name: "Practician", tagline: "Cum îl folosesc corect" },
    Level { name: "Avansat", tagline: "De ce funcționează așa" },
    Level { name: "Expert", tagline: "Cum funcționează pe dinăuntru" },
];

pub struct Topic {
    pub slug: &'static str,
    pub title: &'static str,
    pub levels: [&'static str; 4],
}

macro_rules! topic {
    ($slug:literal, $title:literal) => {
        Topic {
            slug: $slug,
            title: $title,
            levels: [
                include_str!(concat!("../content/", $slug, "/1.md")),
                include_str!(concat!("../content/", $slug, "/2.md")),
                include_str!(concat!("../content/", $slug, "/3.md")),
                include_str!(concat!("../content/", $slug, "/4.md")),
            ],
        }
    };
}

pub static TOPICS: &[Topic] = &[
    topic!("variabile-tipuri", "Variabile și tipuri"),
    topic!("control-flow", "Control flow"),
    topic!("functii", "Funcții"),
    topic!("ownership", "Ownership și borrowing"),
    topic!("structs-enums", "Structs și enums"),
    topic!("pattern-matching", "Pattern matching"),
    topic!("erori", "Gestionarea erorilor"),
    topic!("colectii", "Colecții"),
    topic!("traits-generics", "Traits și generics"),
    topic!("lifetimes", "Lifetimes"),
    topic!("closures-iteratori", "Closures și iteratori"),
    topic!("module-cargo", "Module și Cargo"),
    topic!("smart-pointers", "Smart pointers"),
    topic!("concurenta", "Concurență"),
    topic!("async", "Async/await"),
    topic!("macro-uri", "Macro-uri"),
    topic!("unsafe-ffi", "Unsafe și FFI"),
    topic!("testare", "Testare"),
];
