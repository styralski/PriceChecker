use anyhow::Context;
use mongodb::{
    Client, Collection,
    bson::{Document, doc, to_document},
    options::ClientOptions,
};
use price_check_shared::{Offer, normalize_query};

#[derive(Clone)]
pub struct OfferRepository {
    collection: Collection<Document>,
}

impl OfferRepository {
    pub async fn connect(uri: &str, db_name: &str, collection_name: &str) -> anyhow::Result<Self> {
        let mut options = ClientOptions::parse(uri)
            .await
            .context("failed to parse Mongo URI")?;
        options.app_name = Some("pricechecker-rust".to_string());

        let client = Client::with_options(options).context("failed to create Mongo client")?;
        let collection = client
            .database(db_name)
            .collection::<Document>(collection_name);

        Ok(Self { collection })
    }

    pub async fn find_by_query(&self, raw_query: &str) -> anyhow::Result<Vec<Offer>> {
        let normalized = normalize_query(raw_query);
        let mut cursor = self
            .collection
            .find(doc! { "normalized_query": normalized })
            .sort(doc! { "timestamp": -1 })
            .await
            .context("mongo find failed")?;

        let mut offers = Vec::new();
        while cursor
            .advance()
            .await
            .context("mongo cursor advance failed")?
        {
            let doc = cursor
                .deserialize_current()
                .context("mongo document decode failed")?;
            if let Ok(offer) = mongodb::bson::from_document::<Offer>(doc) {
                offers.push(offer);
            }
        }

        Ok(offers)
    }

    pub async fn list_offers(
        &self,
        name_query: Option<&str>,
        sort_asc: bool,
    ) -> anyhow::Result<Vec<Offer>> {
        let mut filter = doc! {};

        if let Some(name) = name_query.map(str::trim).filter(|s| !s.is_empty()) {
            filter.insert("product_title", doc! { "$regex": name, "$options": "i" });
        }

        let sort_direction = if sort_asc { 1 } else { -1 };
        let mut cursor = self
            .collection
            .find(filter)
            .sort(doc! { "timestamp": sort_direction })
            .await
            .context("mongo list offers failed")?;

        let mut offers = Vec::new();
        while cursor
            .advance()
            .await
            .context("mongo cursor advance failed")?
        {
            let doc = cursor
                .deserialize_current()
                .context("mongo document decode failed")?;
            if let Ok(offer) = mongodb::bson::from_document::<Offer>(doc) {
                offers.push(offer);
            }
        }

        Ok(offers)
    }

    pub async fn insert_many(&self, offers: &[Offer]) -> anyhow::Result<()> {
        if offers.is_empty() {
            return Ok(());
        }

        let docs = offers
            .iter()
            .map(to_document)
            .collect::<Result<Vec<_>, _>>()
            .context("failed to map offers to documents")?;

        self.collection
            .insert_many(docs)
            .await
            .context("mongo insert_many failed")?;

        Ok(())
    }

    pub async fn ensure_indexes(&self) -> anyhow::Result<()> {
        use mongodb::{IndexModel, bson::doc, options::IndexOptions};

        let normalized_query_index = IndexModel::builder()
            .keys(doc! { "normalized_query": 1 })
            .options(Some(
                IndexOptions::builder()
                    .name("normalized_query_idx".to_string())
                    .build(),
            ))
            .build();

        let keywords_index = IndexModel::builder()
            .keys(doc! { "keywords": 1 })
            .options(Some(
                IndexOptions::builder()
                    .name("keywords_idx".to_string())
                    .build(),
            ))
            .build();

        self.collection.create_index(normalized_query_index).await?;
        self.collection.create_index(keywords_index).await?;
        Ok(())
    }
}
