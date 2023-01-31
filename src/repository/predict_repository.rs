use actix_web::web::Bytes;
use crate::obj::data::Data;
use crate::obj::score::Score;
use std::str;

pub fn chatgpt_predict(data: Data) -> Score{
    
    Score{score: Some(0.5)}
}

pub fn minist_predict(image: Bytes) -> f64{
    // let image = image::load_from_memory( &*image);
    // print!("{:?}", image);
    0.5
}