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
pub fn home_page() -> impl IntoView {
    let (query, set_query) = signal(String::new());
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);
    let (offers, set_offers) = signal(Vec::<Offer>::new());
    let (from_cache, set_from_cache) = signal(false);

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let q = query.get_untracked();
        if q.trim().is_empty() {
            return;
        }

        set_loading.set(true);
        set_error.set(None);

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
        <section id="home" class="hero reveal delay-1">
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
    }
}
