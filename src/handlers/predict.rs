use actix_web::{web, Error, HttpResponse};
use crate::models::data::Data;

pub async fn chatgpt_predict(_query: web::Json<Data>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}