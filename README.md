# Rust Futures

Aplicație web despre funcționalitățile limbajului Rust, organizată pe 4 niveluri de adâncime:

| Nivel | Nume | Întrebarea la care răspunde |
|---|---|---|
| 1 | Fundamente | Ce este și cum îl folosesc |
| 2 | Practician | Cum îl folosesc corect |
| 3 | Avansat | De ce funcționează așa |
| 4 | Expert | Cum funcționează pe dinăuntru |

Fiecare subiect acoperă toate cele 4 niveluri.

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

Textele sunt în `content/<subiect>/<nivel>.md`. Lista subiectelor este în `src/content.rs`.

Blocurile de cod marcate cu ` ```rust ` primesc un buton „Rulează” care le deschide în Rust Playground:

- ` ```rust,compile_fail ` marchează un exemplu care nu compilează intenționat;
- ` ```rust,ignore ` ascunde butonul de rulare.
