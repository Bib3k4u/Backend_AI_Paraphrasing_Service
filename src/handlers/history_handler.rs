use crate::services::db_service::ParaphrasedTextService;
use actix_web::{web, HttpResponse, Result};

pub async fn get_all_paraphrased_texts(
    db_service: web::Data<ParaphrasedTextService>,
) -> Result<HttpResponse> {
    match db_service.get_all_paraphrased_texts().await {
        Ok(texts) => Ok(HttpResponse::Ok().json(texts)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Database error: {}", e))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use dotenv::dotenv;

    #[actix_web::test]
    async fn test_get_all_paraphrased_texts() {
        dotenv().ok();

        // Initialize test database
        let db = mongodb::Client::with_uri_str(
            std::env::var("MONGODB_URI")
                .unwrap_or_else(|_| "mongodb://localhost:27017".to_string()),
        )
        .await
        .unwrap()
        .database("test_db");

        let db_service = web::Data::new(ParaphrasedTextService::new(db));

        let resp = get_all_paraphrased_texts(db_service).await.unwrap();
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        let body = test::read_body(resp).await;
        let _: Vec<crate::models::ParaphrasedText> = serde_json::from_slice(&body).unwrap();
    }
}
