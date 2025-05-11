use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParaphraseRequest {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParaphraseResponse {
    pub paraphrased_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParaphrasedText {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub original_text: String,
    pub paraphrased_text: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
