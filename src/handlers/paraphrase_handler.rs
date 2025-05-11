use crate::models::{ParaphraseRequest, ParaphraseResponse};
use crate::services::db_service::ParaphrasedTextService;
use crate::services::paraphrase_service;
use actix_web::{web, HttpResponse, Result};

pub async fn paraphrase(
    req: web::Json<ParaphraseRequest>,
    db_service: web::Data<ParaphrasedTextService>,
) -> Result<HttpResponse> {
    match paraphrase_service::paraphrase_text(&req.text).await {
        Ok(paraphrased_text) => {
            // Store in database
            match db_service
                .save_paraphrased_text(req.text.clone(), paraphrased_text.clone())
                .await
            {
                Ok(_) => Ok(HttpResponse::Ok().json(ParaphraseResponse { paraphrased_text })),
                Err(e) => {
                    Ok(HttpResponse::InternalServerError().json(format!("Database error: {}", e)))
                }
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("API error: {}", e))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use dotenv::dotenv;
    #[actix_web::test]
    async fn test_paraphrase_handler() {
        dotenv().ok();

        // Initialize test database
        let db = mongodb::Client::with_uri_str("mongodb://localhost:27017")
            .await
            .unwrap()
            .database("test_db");
        let db_service = web::Data::new(ParaphrasedTextService::new(db));

        let req = ParaphraseRequest {
            text: "Test this text for paraphrasing.".to_string(),
        };

        let resp = paraphrase(web::Json(req), db_service).await.unwrap();
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        let body = test::read_body(resp).await;
        let response: ParaphraseResponse = serde_json::from_slice(&body).unwrap();
        assert!(!response.paraphrased_text.is_empty());
    }
}
