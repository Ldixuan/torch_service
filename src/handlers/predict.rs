use actix_web::{web, Error, HttpResponse, post};
use crate::obj::data::Data;
use crate::repository::predict_repository;

#[post("/chatgpt_predict")] 
pub async fn chatgpt_predict(_query: web::Json<Data>) -> Result<HttpResponse, Error> {
    let score = predict_repository::chatgpt_predict(_query.0);
    let response_body = web::Json(score);
    Ok(HttpResponse::Ok().json(response_body))
}

#[cfg(test)]
mod predict_api_tests{

    use actix_web::{test, App, dev::Service, http::{StatusCode, header}, body};
    use crate::app_config::config_app;
    use crate::obj::score::Score;

    #[actix_web::test]
    pub async fn test_chatgpt_precit(){
        let app = test::init_service(App::new().configure(config_app)).await;
            
        let payload = r#"{"id":12345,"data_type":"fancy","name":"test"}"#.as_bytes();
            
        let req = test::TestRequest::post()
            .uri("/chatgpt_predict")
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .set_payload(payload)
            .to_request();
            
            let resp = app.call(req).await.unwrap();
            assert_eq!(resp.status(), StatusCode::OK);
            let resp_body = resp.into_body();
            let body_bytes = body::to_bytes(resp_body).await.unwrap();
            let score: Score = serde_json::from_slice(&body_bytes).unwrap();
            assert_eq!(score.score.unwrap(), 0.5);
    }
}