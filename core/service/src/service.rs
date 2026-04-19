use std::collections::HashSet;

use chrono::Utc;
use futures::future::join_all;
use price_check_shared::{normalize_query, split_keywords, Offer, SearchResponse};
use reqwest::Client;

use crate::{db::OfferRepository, scrapers::{default_scrapers, RawOffer}};

#[derive(Clone)]
pub struct SearchService {
    repo: OfferRepository,
    http: Client,
}

impl SearchService {
    pub fn new(repo: OfferRepository, http: Client) -> Self {
        Self { repo, http }
    }

    pub async fn search(&self, query: &str) -> anyhow::Result<SearchResponse> {
        let normalized = normalize_query(query);
        let cached = self.repo.find_by_query(&normalized).await?;

        if !cached.is_empty() {
            return Ok(SearchResponse {
                query: normalized,
                from_cache: true,
                offers: cached,
            });
        }

        let keywords = split_keywords(&normalized);
        let scrapers = default_scrapers();
        let mut tasks = Vec::new();

        for scraper in scrapers {
            let client = self.http.clone();
            let q = normalized.clone();
            tasks.push(tokio::spawn(async move { scraper.scrape(&client, &q).await.unwrap_or_default() }));
        }

        let task_results = join_all(tasks).await;
        let mut raw_offers = Vec::new();
        for result in task_results {
            if let Ok(items) = result {
                raw_offers.extend(items);
            }
        }

        let offers = deduplicate(raw_offers)
            .into_iter()
            .map(|o| Offer {
                product_title: o.product_title,
                shop_name: o.shop_name,
                price: o.price,
                link: o.link,
                timestamp: Utc::now().to_rfc3339(),
                normalized_query: normalized.clone(),
                keywords: keywords.clone(),
                source: o.source,
            })
            .collect::<Vec<_>>();

        self.repo.insert_many(&offers).await?;

        Ok(SearchResponse {
            query: normalized,
            from_cache: false,
            offers,
        })
    }
}

fn deduplicate(items: Vec<RawOffer>) -> Vec<RawOffer> {
    let mut seen = HashSet::new();
    let mut output = Vec::new();

    for item in items {
        let key = format!("{}|{}", item.shop_name.to_lowercase(), item.link.to_lowercase());
        if seen.insert(key) {
            output.push(item);
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::deduplicate;
    use crate::scrapers::RawOffer;

    #[test]
    fn deduplicates_by_shop_and_link() {
        let data = vec![
            RawOffer {
                product_title: "A".to_string(),
                shop_name: "Shop".to_string(),
                price: "1".to_string(),
                link: "https://x".to_string(),
                source: "shop".to_string(),
            },
            RawOffer {
                product_title: "B".to_string(),
                shop_name: "Shop".to_string(),
                price: "2".to_string(),
                link: "https://x".to_string(),
                source: "shop".to_string(),
            },
        ];

        assert_eq!(deduplicate(data).len(), 1);
    }
}
