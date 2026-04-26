use leptos::prelude::*;
use leptos::task::spawn_local;
use price_check_shared::Offer;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct OffersApiResponse {
    ok: bool,
    data: Option<Vec<Offer>>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct OffersRequest {
    query: Option<String>,
    sort: String,
}

fn format_date_only(timestamp: &str) -> String {
    timestamp.split('T').next().unwrap_or(timestamp).to_string()
}

#[component]
pub fn show_all_page() -> impl IntoView {
    let (search_input, set_search_input) = signal(String::new());
    let (active_query, set_active_query) = signal(String::new());
    let (sort_asc, set_sort_asc) = signal(false);
    let (date_filter, set_date_filter) = signal(String::new());
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);
    let (offers, set_offers) = signal(Vec::<Offer>::new());

    let filtered_offers = move || {
        let selected_date = date_filter.get();
        offers
            .get()
            .into_iter()
            .filter(|offer| {
                if selected_date.is_empty() {
                    true
                } else {
                    format_date_only(&offer.timestamp) == selected_date
                }
            })
            .collect::<Vec<_>>()
    };

    let load_offers = move |query: String, asc: bool| {
        set_loading.set(true);
        set_error.set(None);

        spawn_local(async move {
            let payload = serde_json::to_string(&OffersRequest {
                query: if query.trim().is_empty() {
                    None
                } else {
                    Some(query.clone())
                },
                sort: if asc {
                    "asc".to_string()
                } else {
                    "desc".to_string()
                },
            })
            .unwrap_or_else(|_| "{}".to_string());

            let result = Request::post("/api/offers")
                .header("Content-Type", "application/json")
                .body(payload)
                .send()
                .await;

            match result {
                Ok(resp) => {
                    if !resp.ok() {
                        set_error.set(Some(
                            "Backend returned an error while loading offers.".to_string(),
                        ));
                        set_offers.set(Vec::new());
                    } else {
                        match resp.json::<OffersApiResponse>().await {
                            Ok(api) if api.ok => {
                                set_offers.set(api.data.unwrap_or_default());
                            }
                            Ok(api) => {
                                set_error.set(Some(api.error.unwrap_or_else(|| {
                                    "Failed to load offers from database".to_string()
                                })));
                                set_offers.set(Vec::new());
                            }
                            Err(_) => {
                                set_error.set(Some("Backend response was not JSON".to_string()));
                                set_offers.set(Vec::new());
                            }
                        }
                    }
                }
                Err(_) => {
                    set_error.set(Some(
                        "Network request failed while loading offers".to_string(),
                    ));
                    set_offers.set(Vec::new());
                }
            }

            set_loading.set(false);
        });
    };

    {
        let load_offers = load_offers.clone();
        Effect::new(move |_| {
            load_offers(String::new(), false);
        });
    }

    let on_submit = {
        let load_offers = load_offers.clone();
        move |ev: web_sys::SubmitEvent| {
            ev.prevent_default();
            let query = search_input.get_untracked();
            set_active_query.set(query.clone());
            load_offers(query, sort_asc.get_untracked());
        }
    };

    let on_sort_asc = {
        let load_offers = load_offers.clone();
        move |_| {
            set_sort_asc.set(true);
            load_offers(active_query.get_untracked(), true);
        }
    };

    let on_sort_desc = {
        let load_offers = load_offers.clone();
        move |_| {
            set_sort_asc.set(false);
            load_offers(active_query.get_untracked(), false);
        }
    };

    view! {
        <section class="panel reveal delay-1">
            <div class="results-head">
                <h2>"All Database Records"</h2>
                <span class="hint">
                    {move || if sort_asc.get() { "Sorted by timestamp: ascending" } else { "Sorted by timestamp: descending" }}
                </span>
            </div>

            <div class="db-toolbar">
                <form class="db-search-form" on:submit=on_submit>
                    <span class="search-icon" aria-hidden="true">
                        <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2">
                            <circle cx="11" cy="11" r="7"></circle>
                            <line x1="16.65" y1="16.65" x2="21" y2="21"></line>
                        </svg>
                    </span>
                    <input
                        type="text"
                        placeholder="Search database"
                        prop:value=move || search_input.get()
                        on:input=move |ev| set_search_input.set(event_target_value(&ev))
                    />
                    <button type="submit" disabled=move || loading.get()>
                        {move || if loading.get() { "Searching..." } else { "Search" }}
                    </button>
                </form>

                <div class="db-date-filter">
                    <label for="timestamp-filter">"Filter date"</label>
                    <input
                        id="timestamp-filter"
                        type="date"
                        prop:value=move || date_filter.get()
                        on:input=move |ev| set_date_filter.set(event_target_value(&ev))
                    />
                </div>

                <div class="sort-buttons" aria-label="Sort by timestamp">
                    <button
                        type="button"
                        class=move || if sort_asc.get() { "sort-btn active" } else { "sort-btn" }
                        on:click=on_sort_asc
                        title="Sort ascending"
                    >
                        <svg viewBox="0 0 24 24" width="15" height="15" fill="currentColor" aria-hidden="true">
                            <path d="M12 6L6 14H18L12 6Z"></path>
                        </svg>
                    </button>

                    <button
                        type="button"
                        class=move || if !sort_asc.get() { "sort-btn active" } else { "sort-btn" }
                        on:click=on_sort_desc
                        title="Sort descending"
                    >
                        <svg viewBox="0 0 24 24" width="15" height="15" fill="currentColor" aria-hidden="true">
                            <path d="M12 18L18 10H6L12 18Z"></path>
                        </svg>
                    </button>
                </div>
            </div>

            <Show when=move || error.get().is_some()>
                <div class="status status-warn">{move || error.get().unwrap_or_default()}</div>
            </Show>

            <Show when=move || !loading.get() && filtered_offers().is_empty()>
                <div class="empty">"No records found in the database for this filter."</div>
            </Show>

            <Show when=move || !filtered_offers().is_empty()>
                <div class="table-wrap reveal delay-2">
                    <table class="offers-table">
                        <thead>
                            <tr>
                                <th>"Product"</th>
                                <th>"Shop"</th>
                                <th>"Price"</th>
                                <th>"Date"</th>
                                <th>"Query"</th>
                                <th>"Source"</th>
                                <th>"Link"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <For
                                each=move || filtered_offers()
                                key=|o| format!("{}:{}:{}", o.shop_name, o.link, o.timestamp)
                                children=move |offer| {
                                    let Offer {
                                        product_title,
                                        shop_name,
                                        price,
                                        timestamp,
                                        normalized_query,
                                        source,
                                        link,
                                        ..
                                    } = offer;

                                    view! {
                                        <tr>
                                            <td>{product_title}</td>
                                            <td>{shop_name}</td>
                                            <td>{price}</td>
                                            <td>{format_date_only(&timestamp)}</td>
                                            <td>{normalized_query}</td>
                                            <td>{source}</td>
                                            <td>
                                                <a class="open-link" href=link target="_blank" rel="noopener noreferrer">
                                                    "Open"
                                                </a>
                                            </td>
                                        </tr>
                                    }
                                }
                            />
                        </tbody>
                    </table>
                </div>
            </Show>
        </section>
    }
}
