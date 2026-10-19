//! Catalogul de conținut: limbile, nivelurile și subiectele aplicației.
//!
//! Fiecare subiect are câte un fișier Markdown pentru fiecare nivel și limbă:
//! `content/<slug>/<nivel>.md` (română) și `content/<slug>/<nivel>.en.md`
//! (engleză). Fișierele sunt incluse în binar la compilare, deci un fișier
//! lipsă e o eroare de compilare, nu o pagină goală.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Lang {
    Ro,
    En,
}

impl Lang {
    pub const ALL: [Lang; 2] = [Lang::Ro, Lang::En];

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn code(self) -> &'static str {
        match self {
            Lang::Ro => "ro",
            Lang::En => "en",
        }
    }

    pub fn other(self) -> Lang {
        match self {
            Lang::Ro => Lang::En,
            Lang::En => Lang::Ro,
        }
    }
}

pub struct Level {
    name: [&'static str; 2],
    tagline: [&'static str; 2],
}

impl Level {
    pub fn name(&self, lang: Lang) -> &'static str {
        self.name[lang.index()]
    }

    pub fn tagline(&self, lang: Lang) -> &'static str {
        self.tagline[lang.index()]
    }
}

pub const LEVELS: [Level; 4] = [
    Level {
        name: ["Fundamente", "Fundamentals"],
        tagline: ["Ce este și cum îl folosesc", "What it is and how to use it"],
    },
    Level {
        name: ["Practician", "Practitioner"],
        tagline: ["Cum îl folosesc corect", "How to use it well"],
    },
    Level {
        name: ["Avansat", "Advanced"],
        tagline: ["De ce funcționează așa", "Why it works this way"],
    },
    Level {
        name: ["Expert", "Expert"],
        tagline: ["Cum funcționează pe dinăuntru", "How it works under the hood"],
    },
];

pub struct Topic {
    pub slug: &'static str,
    title: [&'static str; 2],
    pages: [[&'static str; 4]; 2],
}

impl Topic {
    pub fn title(&self, lang: Lang) -> &'static str {
        self.title[lang.index()]
    }

    /// Markdown-ul paginii pentru nivelul dat (de la 0 la 3).
    pub fn page(&self, lang: Lang, level: usize) -> &'static str {
        self.pages[lang.index()][level]
    }
}

macro_rules! topic {
    ($slug:literal, $ro:literal, $en:literal) => {
        Topic {
            slug: $slug,
            title: [$ro, $en],
            pages: [
                [
                    include_str!(concat!("../content/", $slug, "/1.md")),
                    include_str!(concat!("../content/", $slug, "/2.md")),
                    include_str!(concat!("../content/", $slug, "/3.md")),
                    include_str!(concat!("../content/", $slug, "/4.md")),
                ],
                [
                    include_str!(concat!("../content/", $slug, "/1.en.md")),
                    include_str!(concat!("../content/", $slug, "/2.en.md")),
                    include_str!(concat!("../content/", $slug, "/3.en.md")),
                    include_str!(concat!("../content/", $slug, "/4.en.md")),
                ],
            ],
        }
    };
}

pub static TOPICS: &[Topic] = &[
    topic!("variabile-tipuri", "Variabile și tipuri", "Variables and types"),
    topic!("control-flow", "Control flow", "Control flow"),
    topic!("functii", "Funcții", "Functions"),
    topic!("ownership", "Ownership și borrowing", "Ownership and borrowing"),
    topic!("structs-enums", "Structs și enums", "Structs and enums"),
    topic!("pattern-matching", "Pattern matching", "Pattern matching"),
    topic!("erori", "Gestionarea erorilor", "Error handling"),
    topic!("colectii", "Colecții", "Collections"),
    topic!("traits-generics", "Traits și generics", "Traits and generics"),
    topic!("lifetimes", "Lifetimes", "Lifetimes"),
    topic!("closures-iteratori", "Closures și iteratori", "Closures and iterators"),
    topic!("module-cargo", "Module și Cargo", "Modules and Cargo"),
    topic!("smart-pointers", "Smart pointers", "Smart pointers"),
    topic!("concurenta", "Concurență", "Concurrency"),
    topic!("async", "Async/await", "Async/await"),
    topic!("macro-uri", "Macro-uri", "Macros"),
    topic!("unsafe-ffi", "Unsafe și FFI", "Unsafe and FFI"),
    topic!("testare", "Testare", "Testing"),
];
