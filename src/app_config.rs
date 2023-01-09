use actix_web::web;

use crate::handlers::predict;


pub fn config_app(cfg: &mut web::ServiceConfig) {
    // domain includes: /products/{product_id}/parts/{part_id}
    // cfg.service(
        // web::scope("/chatgpt_predict")
            // .service(
                // web::resource("")
                    // .route(web::post().to(predict::chatgpt_predict))
            // )
            // 
    // );
    cfg
    .service(predict::chatgpt_predict);
}