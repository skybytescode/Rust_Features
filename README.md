# Rust Features

Aplicație web despre funcționalitățile limbajului Rust, organizată pe 4 niveluri de adâncime:

| Nivel | Nume | Întrebarea la care răspunde |
|---|---|---|
| 1 | Fundamente | Ce este și cum îl folosesc |
| 2 | Practician | Cum îl folosesc corect |
| 3 | Avansat | De ce funcționează așa |
| 4 | Expert | Cum funcționează pe dinăuntru |

Fiecare subiect acoperă toate cele 4 niveluri. Site-ul e disponibil în română și în engleză (butonul **EN / RO** din bara laterală).

## Tech stack

- [Leptos](https://leptos.dev) (CSR), compilat în WebAssembly
- [Trunk](https://trunkrs.dev) pentru build
- GitHub Actions + GitHub Pages pentru publicare

## Rulare locală

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
trunk serve --open
```

## Conținut

Textele sunt în `content/<subiect>/<nivel>.md` (română) și `content/<subiect>/<nivel>.en.md` (engleză). Lista subiectelor și titlurile lor în ambele limbi sunt în `src/content.rs`, iar textele interfeței în `src/i18n.rs`.

URL-urile în română au forma `#/<subiect>/<nivel>`, iar cele în engleză `#/en/<subiect>/<nivel>`.

Blocurile de cod marcate cu ` ```rust ` primesc un buton „Rulează” care le deschide în Rust Playground:

- ` ```rust,compile_fail ` marchează un exemplu care nu compilează intenționat;
- ` ```rust,ignore ` ascunde butonul de rulare.
