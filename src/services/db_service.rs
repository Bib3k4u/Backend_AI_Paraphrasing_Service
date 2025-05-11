use crate::models::ParaphrasedText;
use chrono::Utc;
use futures::stream::TryStreamExt;
use mongodb::{Collection, Database};

pub struct ParaphrasedTextService {
    collection: Collection<ParaphrasedText>,
}

impl ParaphrasedTextService {
    pub fn new(db: Database) -> Self {
        let collection = db.collection("paraphrased_texts");
        Self { collection }
    }

    pub async fn save_paraphrased_text(
        &self,
        original_text: String,
        paraphrased_text: String,
    ) -> Result<ParaphrasedText, mongodb::error::Error> {
        let text = ParaphrasedText {
            id: None,
            original_text,
            paraphrased_text,
            created_at: Utc::now(),
        };

        self.collection.insert_one(&text, None).await?;
        Ok(text)
    }

    pub async fn get_all_paraphrased_texts(
        &self,
    ) -> Result<Vec<ParaphrasedText>, mongodb::error::Error> {
        let mut cursor = self.collection.find(None, None).await?;
        let mut texts = Vec::new();

        while let Some(text) = cursor.try_next().await? {
            texts.push(text);
        }

        Ok(texts)
    }
}
