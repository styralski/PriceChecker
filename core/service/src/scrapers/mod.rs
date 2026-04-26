use anyhow::Context;
use async_trait::async_trait;
use reqwest::{Client, header};
use scraper::{Html, Selector};
use tokio::time::{Duration, sleep};

#[derive(Debug, Clone)]
pub struct RawOffer {
    pub product_title: String,
    pub shop_name: String,
    pub price: String,
    pub link: String,
    pub source: String,
}

#[async_trait]
pub trait SourceScraper: Send + Sync {
    async fn scrape(&self, client: &Client, query: &str) -> anyhow::Result<Vec<RawOffer>>;
}

pub fn build_client() -> anyhow::Result<Client> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36",
        ),
    );

    Ok(Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(12))
        .build()
        .context("failed to build reqwest client")?)
}

fn parse_selector(selector: &str) -> Option<Selector> {
    Selector::parse(selector).ok()
}

fn text(node: scraper::ElementRef<'_>) -> String {
    node.text()
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn absolutize(base: &str, maybe_relative: &str) -> String {
    if maybe_relative.starts_with("http://") || maybe_relative.starts_with("https://") {
        return maybe_relative.to_string();
    }
    format!("{}{}", base.trim_end_matches('/'), maybe_relative)
}

pub struct ShopScraper {
    pub shop_name: &'static str,
    pub source: &'static str,
    pub search_url_template: &'static str,
    pub item_selector: &'static str,
    pub title_selector: &'static str,
    pub price_selector: &'static str,
    pub link_selector: &'static str,
    pub base_url: &'static str,
}

#[async_trait]
impl SourceScraper for ShopScraper {
    async fn scrape(&self, client: &Client, query: &str) -> anyhow::Result<Vec<RawOffer>> {
        let encoded = urlencoding::encode(query);
        let url = self.search_url_template.replace("{}", &encoded);

        let body = client
            .get(url)
            .send()
            .await
            .with_context(|| format!("request failed for {}", self.shop_name))?
            .text()
            .await
            .with_context(|| format!("failed to read body for {}", self.shop_name))?;

        let offers = {
            let document = Html::parse_document(&body);
            let item_sel = match parse_selector(self.item_selector) {
                Some(v) => v,
                None => return Ok(vec![]),
            };
            let title_sel = match parse_selector(self.title_selector) {
                Some(v) => v,
                None => return Ok(vec![]),
            };
            let price_sel = match parse_selector(self.price_selector) {
                Some(v) => v,
                None => return Ok(vec![]),
            };
            let link_sel = match parse_selector(self.link_selector) {
                Some(v) => v,
                None => return Ok(vec![]),
            };

            let mut offers = Vec::new();
            for item in document.select(&item_sel).take(5) {
                let title = item.select(&title_sel).next().map(text).unwrap_or_default();
                let price = item.select(&price_sel).next().map(text).unwrap_or_default();
                let link = item
                    .select(&link_sel)
                    .next()
                    .and_then(|n| n.value().attr("href"))
                    .map(|l| absolutize(self.base_url, l))
                    .unwrap_or_default();

                if title.is_empty() || link.is_empty() {
                    continue;
                }

                offers.push(RawOffer {
                    product_title: title,
                    shop_name: self.shop_name.to_string(),
                    price,
                    link,
                    source: self.source.to_string(),
                });
            }

            offers
        };

        sleep(Duration::from_millis(300)).await;
        Ok(offers)
    }
}

pub struct GoogleScraper;

#[async_trait]
impl SourceScraper for GoogleScraper {
    async fn scrape(&self, client: &Client, query: &str) -> anyhow::Result<Vec<RawOffer>> {
        let encoded = urlencoding::encode(query);
        let url = format!("https://www.google.com/search?q={}", encoded);
        let body = client
            .get(url)
            .send()
            .await
            .context("google request failed")?
            .text()
            .await
            .context("google response decode failed")?;

        let offers = {
            let document = Html::parse_document(&body);
            let item_sel = match parse_selector("div.g") {
                Some(v) => v,
                None => return Ok(vec![]),
            };
            let title_sel = match parse_selector("h3") {
                Some(v) => v,
                None => return Ok(vec![]),
            };
            let link_sel = match parse_selector("a") {
                Some(v) => v,
                None => return Ok(vec![]),
            };

            let mut offers = Vec::new();
            for item in document.select(&item_sel).take(3) {
                let title = item.select(&title_sel).next().map(text).unwrap_or_default();
                let link = item
                    .select(&link_sel)
                    .next()
                    .and_then(|n| n.value().attr("href"))
                    .unwrap_or_default()
                    .to_string();

                if title.is_empty() || link.is_empty() || !link.starts_with("http") {
                    continue;
                }

                offers.push(RawOffer {
                    product_title: title,
                    shop_name: "Google Results".to_string(),
                    price: "N/A".to_string(),
                    link,
                    source: "google".to_string(),
                });
            }

            offers
        };

        sleep(Duration::from_millis(300)).await;
        Ok(offers)
    }
}

pub fn default_scrapers() -> Vec<Box<dyn SourceScraper>> {
    vec![
        Box::new(ShopScraper {
            shop_name: "Ceneo.pl",
            source: "ceneo",
            search_url_template: "https://www.ceneo.pl/szukaj-{}",
            item_selector: "div.js_products-list-main",
            title_selector: "a span",
            price_selector: "span.price",
            link_selector: "a",
            base_url: "https://www.ceneo.pl",
        }),
        Box::new(ShopScraper {
            shop_name: "Morele.net",
            source: "morele",
            search_url_template: "https://www.morele.net/wyszukiwarka/?q={}&d=0",
            item_selector: "div.cat-product-content",
            title_selector: "a",
            price_selector: "div.price-new",
            link_selector: "a",
            base_url: "https://www.morele.net",
        }),
        Box::new(ShopScraper {
            shop_name: "Media Expert",
            source: "media_expert",
            search_url_template: "https://www.mediaexpert.pl/search?query[menu_item]=&query[querystring]={}",
            item_selector: "div.offer-box",
            title_selector: "a.spark-link",
            price_selector: "span.whole",
            link_selector: "a.spark-link",
            base_url: "https://www.mediaexpert.pl",
        }),
        Box::new(ShopScraper {
            shop_name: "Komputronik",
            source: "komputronik",
            search_url_template: "https://www.komputronik.pl/search/category/1?query={}",
            item_selector: "div.tests-product-entry",
            title_selector: "a",
            price_selector: "div.text-3xl",
            link_selector: "a.product-image",
            base_url: "https://www.komputronik.pl",
        }),
        Box::new(ShopScraper {
            shop_name: "X-Kom",
            source: "xkom",
            search_url_template: "https://www.x-kom.pl/szukaj?q={}",
            item_selector: "div.sc-f5aee401-0",
            title_selector: "h3.sc-a8d94d6e-0 span",
            price_selector: "span[data-name='productPrice']",
            link_selector: "a",
            base_url: "https://www.x-kom.pl",
        }),
        Box::new(GoogleScraper),
    ]
}
