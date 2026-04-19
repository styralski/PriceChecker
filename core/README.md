# PriceChecker Rust Stack (WIP)

Minimal migration target from Django to Rust + Leptos + MongoDB.

## What is implemented now

- Shared Rust model for offers and search payloads.
- Backend search flow (DB-first):
  - search query
  - check MongoDB by normalized query
  - if found: return cached offers
  - if not found: scrape configured sources, save, return
- Scraper adapters:
  - Ceneo
  - Morele
  - Media Expert
  - Komputronik
  - X-Kom
  - Google search results (up to 3 entries, price can be `N/A`)
- Leptos frontend starter app that calls backend `/api/search` and renders result table.

## Current architecture

- `shared` crate: data contracts and normalization.
- `backend` crate: Axum API + Mongo repository + scrapers + orchestration.
- `frontend` crate: Leptos CSR app (Trunk).

## Required env (backend)

Add to `.env` and adjust values if needed:

- `MONGO_URI`
- `MONGO_DB`
- `MONGO_COLLECTION`
- `RUST_LOG`
- optional: `BIND_ADDR`

## Run backend

```powershell
cd core
cargo run -p price-check-service
```

Backend listens on `http://127.0.0.1:8080` by default.

## Run frontend

Install Trunk if needed:

```powershell
cargo install trunk
```

Run frontend:

```powershell
cd rust-stack/ui
trunk serve --open
```

Frontend calls backend at `http://127.0.0.1:8080/api/search`.

## Notes

- No re-scrape logic is included (as requested).
- No Celery/Redis/task queue in this stack.
- Google scraping is best-effort and can fail due SERP changes/rate limiting.
- CSS selectors are intentionally simple for v1 and may need tuning.
