use leptos::prelude::*;
use leptos::task::spawn_local;
use price_check_shared::{Offer, SearchRequest, SearchResponse};
use reqwasm::http::Request;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ApiResponse {
    ok: bool,
    data: Option<SearchResponse>,
    error: Option<String>,
}

fn demo_offers(query: &str) -> Vec<Offer> {
    let q = query.trim();
    let normalized = if q.is_empty() { "apple watch 10" } else { q };

    vec![
        Offer {
            product_title: format!("{normalized} - Midnight 42mm"),
            shop_name: "DealWave".to_string(),
            price: "1499 PLN".to_string(),
            link: "https://example.com/dealwave".to_string(),
            timestamp: "demo".to_string(),
            normalized_query: normalized.to_string(),
            keywords: vec!["smartwatch".to_string(), "apple".to_string()],
            source: "demo".to_string(),
        },
        Offer {
            product_title: format!("{normalized} - Sport Band Edition"),
            shop_name: "PixelMarket".to_string(),
            price: "1549 PLN".to_string(),
            link: "https://example.com/pixelmarket".to_string(),
            timestamp: "demo".to_string(),
            normalized_query: normalized.to_string(),
            keywords: vec!["watch".to_string(), "deal".to_string()],
            source: "demo".to_string(),
        },
        Offer {
            product_title: format!("{normalized} - GPS + Cellular"),
            shop_name: "PriceRocket".to_string(),
            price: "1699 PLN".to_string(),
            link: "https://example.com/pricerocket".to_string(),
            timestamp: "demo".to_string(),
            normalized_query: normalized.to_string(),
            keywords: vec!["cellular".to_string(), "apple watch".to_string()],
            source: "demo".to_string(),
        },
    ]
}

#[component]
pub fn App() -> impl IntoView {
    let (query, set_query) = signal(String::new());
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);
    let (offers, set_offers) = signal(Vec::<Offer>::new());
    let (from_cache, set_from_cache) = signal(false);
    let (menu_open, set_menu_open) = signal(false);

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let q = query.get_untracked();
        if q.trim().is_empty() {
            return;
        }

        set_loading.set(true);
        set_error.set(None);
        set_menu_open.set(false);

        spawn_local(async move {
            let payload = serde_json::to_string(&SearchRequest { query: q.clone() })
                .unwrap_or_else(|_| "{}".to_string());

            let result = Request::post("/api/search")
                .header("Content-Type", "application/json")
                .body(payload)
                .send()
                .await;

            match result {
                Ok(resp) => {
                    if !resp.ok() {
                        set_from_cache.set(false);
                        set_offers.set(demo_offers(&q));
                        set_error.set(Some(
                            "Backend is offline, showing demo offers so you can work on UI.".to_string(),
                        ));
                    } else {
                        let parsed = resp.json::<ApiResponse>().await;
                        match parsed {
                            Ok(api) if api.ok => {
                                if let Some(data) = api.data {
                                    set_from_cache.set(data.from_cache);
                                    set_offers.set(data.offers);
                                } else {
                                    set_error.set(Some("Missing response data".to_string()));
                                }
                            }
                            Ok(api) => {
                                set_error.set(Some(
                                    api.error.unwrap_or_else(|| "Search failed".to_string()),
                                ));
                            }
                            Err(_) => {
                                set_from_cache.set(false);
                                set_offers.set(demo_offers(&q));
                                set_error.set(Some(
                                    "Backend response was not JSON, showing demo offers.".to_string(),
                                ));
                            }
                        }
                    }
                }
                Err(_) => {
                    set_from_cache.set(false);
                    set_offers.set(demo_offers(&q));
                    set_error.set(Some(
                        "Network request failed, showing demo offers.".to_string(),
                    ));
                }
            }

            set_loading.set(false);
        });
    };

    view! {
        <main class="pc-page">
            <style>
                {r#"
                @import url('https://fonts.googleapis.com/css2?family=Fraunces:opsz,wght@9..144,700&family=Space+Grotesk:wght@400;500;700&display=swap');

                :root {
                    --bg-a: #f4f8f2;
                    --bg-b: #e7f3ea;
                    --ink: #10231a;
                    --muted: #476054;
                    --accent: #1c7a46;
                    --accent-deep: #0f5a34;
                    --card: rgba(255, 255, 255, 0.84);
                    --line: rgba(17, 60, 38, 0.12);
                    --warn-bg: #fff7e9;
                    --warn-ink: #7e4d00;
                }

                * {
                    box-sizing: border-box;
                }

                body {
                    margin: 0;
                    font-family: 'Space Grotesk', sans-serif;
                    color: var(--ink);
                    background:
                        radial-gradient(circle at 15% 18%, rgba(28, 122, 70, 0.16) 0%, transparent 34%),
                        radial-gradient(circle at 90% 8%, rgba(35, 135, 92, 0.18) 0%, transparent 30%),
                        linear-gradient(160deg, var(--bg-a) 0%, var(--bg-b) 100%);
                    min-height: 100vh;
                }

                .pc-page {
                    max-width: 1120px;
                    margin: 0 auto;
                    padding: 1.2rem 1rem 3rem;
                    position: relative;
                }

                .topbar {
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                    gap: 1rem;
                    padding: 0.65rem 0.8rem;
                    border: 1px solid var(--line);
                    border-radius: 16px;
                    backdrop-filter: blur(6px);
                    background: rgba(255, 255, 255, 0.72);
                }

                .brand {
                    display: flex;
                    align-items: center;
                    gap: 0.75rem;
                    text-decoration: none;
                    color: inherit;
                }

                .brand-mark {
                    width: 42px;
                    height: 42px;
                    border-radius: 11px;
                    border: 2px solid var(--accent);
                    display: grid;
                    place-items: center;
                    font-weight: 700;
                    letter-spacing: 0.02em;
                    color: var(--accent-deep);
                    background: white;
                }

                .brand-text {
                    font-family: 'Fraunces', serif;
                    font-size: clamp(1.1rem, 2vw, 1.4rem);
                    letter-spacing: 0.01em;
                }

                .desktop-nav {
                    display: flex;
                    align-items: center;
                    gap: 1rem;
                }

                .desktop-nav a,
                .mobile-nav a {
                    text-decoration: none;
                    color: var(--ink);
                    font-weight: 600;
                    opacity: 0.88;
                }

                .desktop-nav a:hover,
                .mobile-nav a:hover {
                    opacity: 1;
                    color: var(--accent-deep);
                }

                .menu-btn {
                    display: none;
                    border: 1px solid var(--line);
                    background: white;
                    color: var(--ink);
                    font-weight: 600;
                    border-radius: 999px;
                    padding: 0.42rem 0.9rem;
                    cursor: pointer;
                }

                .mobile-nav {
                    margin-top: 0.7rem;
                    border: 1px solid var(--line);
                    border-radius: 14px;
                    background: rgba(255, 255, 255, 0.92);
                    padding: 0.6rem 0.8rem;
                    display: flex;
                    gap: 1rem;
                }

                .hero {
                    margin-top: 1.2rem;
                    border: 1px solid var(--line);
                    border-radius: 20px;
                    padding: clamp(1rem, 2.8vw, 2rem);
                    background:
                        linear-gradient(128deg, rgba(255, 255, 255, 0.95), rgba(233, 248, 238, 0.95));
                }

                .hero h1 {
                    margin: 0;
                    font-family: 'Fraunces', serif;
                    font-size: clamp(1.7rem, 4.2vw, 3rem);
                    line-height: 1.08;
                    max-width: 18ch;
                }

                .hero p {
                    margin: 0.9rem 0 0;
                    color: var(--muted);
                    font-size: clamp(0.98rem, 2vw, 1.1rem);
                    max-width: 64ch;
                }

                .panel {
                    margin-top: 1rem;
                    border: 1px solid var(--line);
                    border-radius: 20px;
                    background: var(--card);
                    padding: 1rem;
                    backdrop-filter: blur(5px);
                }

                .search-form {
                    display: flex;
                    flex-direction: column;
                    gap: 0.55rem;
                }

                .search-form label {
                    font-weight: 700;
                }

                .input-row {
                    display: flex;
                    gap: 0.55rem;
                }

                .input-row input {
                    flex: 1;
                    min-width: 0;
                    border: 1px solid #bfd2c3;
                    border-radius: 12px;
                    background: white;
                    padding: 0.78rem 0.95rem;
                    font-size: 0.98rem;
                    outline: none;
                }

                .input-row input:focus {
                    border-color: var(--accent);
                    box-shadow: 0 0 0 4px rgba(28, 122, 70, 0.12);
                }

                .input-row button {
                    border: 0;
                    border-radius: 12px;
                    background: linear-gradient(135deg, var(--accent), var(--accent-deep));
                    color: white;
                    font-weight: 700;
                    letter-spacing: 0.01em;
                    padding: 0 1rem;
                    cursor: pointer;
                }

                .input-row button[disabled] {
                    opacity: 0.6;
                    cursor: wait;
                }

                .hint {
                    margin: 0;
                    color: #5d7064;
                    font-size: 0.92rem;
                }

                .status {
                    margin-top: 0.8rem;
                    padding: 0.65rem 0.8rem;
                    border-radius: 11px;
                    font-weight: 500;
                }

                .status-warn {
                    background: var(--warn-bg);
                    color: var(--warn-ink);
                    border: 1px solid #f1d7a8;
                }

                .results-head {
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                    gap: 0.8rem;
                    margin-bottom: 0.75rem;
                }

                .results-head h2 {
                    margin: 0;
                    font-family: 'Fraunces', serif;
                    font-size: clamp(1.2rem, 3vw, 1.8rem);
                }

                .tag {
                    display: inline-flex;
                    align-items: center;
                    gap: 0.4rem;
                    border-radius: 999px;
                    font-size: 0.85rem;
                    font-weight: 700;
                    padding: 0.34rem 0.7rem;
                }

                .tag-cache {
                    color: #145f36;
                    background: #e7f8ee;
                    border: 1px solid #bde6cd;
                }

                .tag-fresh {
                    color: #0b4e70;
                    background: #eaf7ff;
                    border: 1px solid #bee6fb;
                }

                .offer-grid {
                    display: grid;
                    grid-template-columns: repeat(3, minmax(0, 1fr));
                    gap: 0.8rem;
                }

                .offer-card {
                    border: 1px solid var(--line);
                    border-radius: 14px;
                    background: white;
                    padding: 0.9rem;
                    display: flex;
                    flex-direction: column;
                    gap: 0.7rem;
                }

                .offer-card h3 {
                    margin: 0;
                    line-height: 1.25;
                    font-size: 1rem;
                }

                .meta-row {
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                    gap: 0.6rem;
                }

                .shop {
                    color: #4f6658;
                    font-weight: 500;
                }

                .price {
                    font-size: 1.02rem;
                    font-weight: 800;
                    color: #0d5f36;
                    white-space: nowrap;
                }

                .open-link {
                    width: fit-content;
                    text-decoration: none;
                    color: white;
                    font-weight: 700;
                    background: #174f33;
                    border-radius: 10px;
                    padding: 0.4rem 0.7rem;
                }

                .open-link:hover {
                    background: #103f27;
                }

                .time {
                    margin: 0;
                    color: #6f8276;
                    font-size: 0.82rem;
                }

                .empty {
                    border: 1px dashed #b6ccb9;
                    border-radius: 14px;
                    padding: 1rem;
                    color: #567060;
                    background: #f9fffb;
                }

                .skeleton-grid {
                    display: grid;
                    grid-template-columns: repeat(3, minmax(0, 1fr));
                    gap: 0.8rem;
                }

                .skeleton-card {
                    height: 150px;
                    border-radius: 14px;
                    border: 1px solid var(--line);
                    background:
                        linear-gradient(
                            110deg,
                            rgba(245, 250, 246, 0.8) 8%,
                            rgba(232, 242, 235, 0.95) 18%,
                            rgba(245, 250, 246, 0.8) 33%
                        );
                    background-size: 200% 100%;
                    animation: shimmer 1.2s linear infinite;
                }

                .reveal {
                    opacity: 0;
                    transform: translateY(12px);
                    animation: reveal 600ms ease forwards;
                }

                .delay-1 { animation-delay: 80ms; }
                .delay-2 { animation-delay: 160ms; }
                .delay-3 { animation-delay: 240ms; }

                @keyframes reveal {
                    to {
                        opacity: 1;
                        transform: translateY(0);
                    }
                }

                @keyframes shimmer {
                    to {
                        background-position-x: -200%;
                    }
                }

                @media (max-width: 900px) {
                    .offer-grid,
                    .skeleton-grid {
                        grid-template-columns: repeat(2, minmax(0, 1fr));
                    }
                }

                @media (max-width: 700px) {
                    .desktop-nav {
                        display: none;
                    }

                    .menu-btn {
                        display: inline-flex;
                    }

                    .input-row {
                        flex-direction: column;
                    }

                    .input-row button {
                        min-height: 42px;
                    }

                    .offer-grid,
                    .skeleton-grid {
                        grid-template-columns: 1fr;
                    }
                }
                "#}
            </style>

            <header class="topbar reveal">
                <a class="brand" href="#">
                    <span class="brand-mark">"PC"</span>
                    <span class="brand-text">"PriceChecker"</span>
                </a>

                <nav class="desktop-nav">
                    <a href="#about">"About"</a>
                    <a href="#results">"Show All"</a>
                </nav>

                <button
                    class="menu-btn"
                    on:click=move |_| set_menu_open.update(|v| *v = !*v)
                >
                    {move || if menu_open.get() { "Close" } else { "Menu" }}
                </button>
            </header>

            <Show when=move || menu_open.get()>
                <nav class="mobile-nav reveal">
                    <a href="#about" on:click=move |_| set_menu_open.set(false)>
                        "About"
                    </a>
                    <a href="#results" on:click=move |_| set_menu_open.set(false)>
                        "Show All"
                    </a>
                </nav>
            </Show>

            <section id="about" class="hero reveal delay-1">
                <h1>"Discover better prices before everyone else."</h1>
                <p>
                    "Compare offers across stores, spot the strongest deal fast, and track whether results come from cache or fresh scraping."
                </p>
            </section>

            <section class="panel reveal delay-2">
                <form class="search-form" on:submit=on_submit>
                    <label>"Choose product"</label>
                    <div class="input-row">
                        <input
                            type="text"
                            placeholder="Which product are you looking for?"
                            prop:value=move || query.get()
                            on:input=move |ev| set_query.set(event_target_value(&ev))
                        />
                        <button type="submit" disabled=move || loading.get()>
                            {move || if loading.get() { "Searching..." } else { "Search Offers" }}
                        </button>
                    </div>
                    <p class="hint">
                        "Try: apple watch 10, xbox series s, nintendo switch oled"
                    </p>
                </form>

                <Show when=move || error.get().is_some()>
                    <div class="status status-warn">{move || error.get().unwrap_or_default()}</div>
                </Show>
            </section>

            <section id="results" class="panel reveal delay-3">
                <div class="results-head">
                    <h2>"Offers"</h2>
                    <Show when=move || !offers.get().is_empty()>
                        <span class=move || if from_cache.get() { "tag tag-cache" } else { "tag tag-fresh" }>
                            {move || if from_cache.get() { "Source: DB cache" } else { "Source: fresh scrape" }}
                        </span>
                    </Show>
                </div>

                <Show when=move || loading.get()>
                    <div class="skeleton-grid">
                        <div class="skeleton-card"></div>
                        <div class="skeleton-card"></div>
                        <div class="skeleton-card"></div>
                    </div>
                </Show>

                <Show when=move || !loading.get() && offers.get().is_empty()>
                    <div class="empty">
                        "No offers yet. Search to load real data, or keep backend off and preview the UI with demo cards."
                    </div>
                </Show>

                <Show when=move || !offers.get().is_empty()>
                    <div class="offer-grid">
                        <For
                            each=move || offers.get()
                            key=|o| format!("{}:{}", o.shop_name, o.link)
                            children=move |offer| {
                                let Offer {
                                    product_title,
                                    shop_name,
                                    price,
                                    link,
                                    timestamp,
                                    ..
                                } = offer;

                                view! {
                                    <article class="offer-card">
                                        <h3>{product_title}</h3>
                                        <div class="meta-row">
                                            <span class="shop">{shop_name}</span>
                                            <span class="price">{price}</span>
                                        </div>
                                        <a class="open-link" href=link target="_blank" rel="noopener noreferrer">
                                            "Open Offer"
                                        </a>
                                        <p class="time">{timestamp}</p>
                                    </article>
                                }
                            }
                        />
                    </div>
                </Show>
            </section>
        </main>
    }
}
