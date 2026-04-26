use leptos::prelude::*;

#[component]
pub fn about_page() -> impl IntoView {
    view! {
        <div class="about-content reveal delay-1">
            <h1 class="about-header">"Price Checker"</h1>
            <p>
                "Price Checker is a web project created to help users avoid unfair pricing."
            </p>
            <p>
                "The main goal of the project is to show how different stores change product prices over long periods, especially around holidays."
            </p>

            <h1 class="about-header">"How It Works"</h1>
            <p>
                "Search for a product you want to buy and wait for results."
            </p>
            <p>
                "If the application does not show results immediately, the product is likely not in the database yet."
            </p>
            <p>
                "In that case, please be patient. The application can automatically run scraping jobs and find matching offers."
            </p>
            <p>
                "Once a product appears in results, it can be tracked periodically so you can identify better times to buy."
            </p>
            <p>
                "Users can compare products across shops and use sorting/filtering views to inspect price changes over time."
            </p>

            <h1 class="about-header">"Donate"</h1>
            <p>
                "The application is free and intended to stay free for the community."
            </p>
            <p>
                "Maintaining scrapers requires ongoing updates whenever shops change their HTML structures."
            </p>
            <p>
                "Support helps keep the service online and expand support for more stores over time."
            </p>
        </div>
    }
}
