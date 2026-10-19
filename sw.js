// Service worker: aplicația instalabilă (PWA) și funcționarea offline.
//
// Tot conținutul e inclus în fișierul WebAssembly, deci odată salvate
// index.html și fișierele generate de Trunk, întreaga aplicație merge offline.
//
// - la instalare: salvează index.html și fișierele cu hash pe care le referă;
// - pagina: întâi rețeaua (versiunea nouă), apoi cache-ul;
// - restul: din cache, actualizat în fundal;
// - după fiecare index.html nou, fișierele cu hash pe care nu le mai referă
//   sunt șterse din cache.
"use strict";

const CACHE = "rust-features-v1";
const SCOPE = new URL("./", self.location).href;
const SHELL = [
  "manifest.webmanifest",
  "icons/apple-touch-icon.png",
  "icons/icon-32.png",
  "icons/icon-192.png",
  "icons/icon-512.png",
  "icons/icon-maskable-512.png",
].map((p) => new URL(p, SCOPE).href);

// Fișierele generate de Trunk au un hash hexa în nume (de obicei 16 cifre, uneori mai puține).
const HASHED = /[\w.-]+-[0-9a-f]{8,}(?:_bg\.wasm|\.js|\.css)/g;

function hashedAssets(html) {
  return [...new Set(html.match(HASHED) || [])].map((name) => new URL(name, SCOPE).href);
}

async function saveIndex(cache, resp) {
  const html = await resp.clone().text();
  await cache.put(SCOPE, resp);
  const wanted = hashedAssets(html);
  // fișierele noi, pe care cache-ul nu le are încă
  await Promise.all(
    wanted.map(async (u) => {
      if (!(await cache.match(u))) await cache.add(u);
    }),
  );
  // fișierele vechi, de la versiunile anterioare
  for (const req of await cache.keys()) {
    HASHED.lastIndex = 0;
    if (HASHED.test(req.url) && !wanted.includes(req.url)) await cache.delete(req);
  }
}

self.addEventListener("install", (ev) => {
  ev.waitUntil(
    (async () => {
      const cache = await caches.open(CACHE);
      await cache.addAll(SHELL);
      await saveIndex(cache, await fetch(SCOPE, { cache: "no-cache" }));
      await self.skipWaiting();
    })(),
  );
});

self.addEventListener("activate", (ev) => {
  ev.waitUntil(
    caches
      .keys()
      .then((keys) => Promise.all(keys.filter((k) => k.startsWith("rust-features-") && k !== CACHE).map((k) => caches.delete(k))))
      .then(() => self.clients.claim()),
  );
});

async function page(req) {
  const cache = await caches.open(CACHE);
  try {
    const resp = await fetch(req);
    if (resp.ok) await saveIndex(cache, resp.clone());
    return resp;
  } catch (err) {
    return (await cache.match(SCOPE)) || Response.error();
  }
}

async function asset(req) {
  const cache = await caches.open(CACHE);
  const hit = await cache.match(req);
  const fresh = fetch(req)
    .then((resp) => {
      if (resp.ok) cache.put(req, resp.clone());
      return resp;
    })
    .catch(() => hit);
  return hit || fresh;
}

self.addEventListener("fetch", (ev) => {
  const req = ev.request;
  if (req.method !== "GET" || !req.url.startsWith(SCOPE)) return;
  ev.respondWith(req.mode === "navigate" ? page(req) : asset(req));
});
