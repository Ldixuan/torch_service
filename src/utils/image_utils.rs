// use image::DynamicImage;
// use image::JPEG;
// use std::error::Error;
// use std::sync::Arc;

// use std::env;

// use actix_web::http::header::CONTENT_TYPE;
// use actix_web::http::{HeaderValue, StatusCode};
// use actix_web::{middleware, web, App, Error as ActixError, HttpResponse, HttpServer,};
// use futures::{Future, Stream};


// use image::ImageError;


// fn get_image(stream: web::Payload) -> DynamicImage {
//     stream
//         .concat2()
//         .from_err()
//         .and_then(move |bytes| web::block(move || image::load_from_memory(&bytes)).from_err())
// }

