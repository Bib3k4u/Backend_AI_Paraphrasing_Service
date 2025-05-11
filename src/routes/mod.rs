use crate::handlers::{history_handler, paraphrase_handler};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::resource("/paraphrase").route(web::post().to(paraphrase_handler::paraphrase)),
            )
            .service(
                web::resource("/history")
                    .route(web::get().to(history_handler::get_all_paraphrased_texts)),
            ),
    );
}
