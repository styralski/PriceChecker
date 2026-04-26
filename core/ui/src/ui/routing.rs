use leptos::prelude::*;
use leptos_router::{
    components::{A, Route, Router, Routes},
    path,
};

use super::pages::{AboutPage, HomePage, ShowAllPage};
use super::styles::APP_STYLES;

#[component]
pub fn app() -> impl IntoView {
    let (menu_open, set_menu_open) = signal(false);

    view! {
        <main class="pc-page">
            <style>{APP_STYLES}</style>

            <Router>
                <header class="topbar reveal">
                    <div class="brand">
                        <span class="brand-mark">"PC"</span>
                        <span class="brand-text">"PriceChecker"</span>
                    </div>

                    <nav class="desktop-nav">
                        <A href="/">"Home"</A>
                        <A href="/about">"About"</A>
                        <A href="/offers">"Show All"</A>
                    </nav>

                    <button class="menu-btn" on:click=move |_| set_menu_open.update(|v| *v = !*v)>
                        {move || if menu_open.get() { "Close" } else { "Menu" }}
                    </button>
                </header>

                <Show when=move || menu_open.get()>
                    <nav class="mobile-nav reveal">
                        <A href="/" on:click=move |_| set_menu_open.set(false)>
                            "Home"
                        </A>
                        <A href="/about" on:click=move |_| set_menu_open.set(false)>
                            "About"
                        </A>
                        <A href="/offers" on:click=move |_| set_menu_open.set(false)>
                            "Show All"
                        </A>
                    </nav>
                </Show>

                <Routes fallback=|| view! { <p class="panel">"Page not found."</p> }>
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/about") view=AboutPage />
                    <Route path=path!("/offers") view=ShowAllPage />
                </Routes>
            </Router>
        </main>
    }
}
