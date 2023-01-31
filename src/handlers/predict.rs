use actix_web::{web::{Bytes}, Error, HttpResponse, post, HttpRequest, HttpMessage, web, dev::{Payload}};
use futures::{StreamExt, stream::Chunks};
use crate::obj::data::Data;
use crate::repository::predict_repository;
use crate::utils::image_utils;
use serde_json::{Map, Value, json};
use actix_form_data;
use actix_multipart::Multipart;
use std::{io, borrow::Borrow};
use futures_util::stream;
use futures_core::stream::{LocalBoxStream, Stream};
use image::{DynamicImage, ImageBuffer};
use show_image::{ImageView, ImageInfo, create_window, error::{UnsupportedImageFormat, ImageDataError}, PixelFormat, Alpha, glam};
use tch::vision::imagenet;
use tch::nn::ModuleT;
use std::ops::Deref;
use bytes::{BytesMut};

#[post("/chatgpt_predict")] 
pub async fn chatgpt_predict(_query: web::Json<Data>) -> Result<HttpResponse, Error> {
    let score = predict_repository::chatgpt_predict(_query.0);
    let response_body = web::Json(score);
    Ok(HttpResponse::Ok().json(response_body))
}





#[post("/minist_predict")] 
pub async fn minist_predict(request: HttpRequest, mut multipart: Multipart) -> Result<HttpResponse, Error>{

    // let headers = request.headers();
    // let mut payload = request.take_payload();
    

    // let chunks = body.as_chunks().0;
    // let mut multipart = Multipart::new::<web::Bytes>(&headers, payload.take().into());

    let chunk_data = match multipart.next().await{
        Some(Ok(mut field)) => {
            let mut b = BytesMut::new();
            loop {
                match field.next().await{
                    Some(Ok(chunk)) => b.extend_from_slice(&chunk),
                    None => break,
                     _ => unreachable!()
                };

            };
            b
            
        }
        _ => unreachable!(),
    };

    let test_data = match multipart.next().await{
        Some(Ok(mut field)) => {
            let mut b = BytesMut::new();
            loop {
                match field.next().await{
                    Some(Ok(chunk)) => b.extend_from_slice(&chunk),
                    None => break,
                     _ => unreachable!()
                };

            };
            b
            
        }
        _ => unreachable!(),
    };

    dbg!(&test_data);

    // let tensor_image = image::load_from_memory(&chunk_data).unwrap();

    // let info = dynamic_image_info(&tensor_image).unwrap();
    // let data = dynamic_image_as_bytes(&tensor_image);
    let tensor_image = imagenet::load_image_from_memory(&chunk_data).unwrap();
    let mut vs = tch::nn::VarStore::new(tch::Device::Cpu);
    let resnet18: Box<dyn ModuleT> = Box::new(tch::vision::resnet::resnet18(&vs.root(), imagenet::CLASS_COUNT));

    let file_name = "./res.ot";
    let weight_path = std::path::Path::new(file_name);
    vs.load(weight_path).unwrap();

    let output = resnet18
        .forward_t(&tensor_image.unsqueeze(0), /*train=*/ false)
        .softmax(-1, tch::Kind::Float);

    for (probability, class) in imagenet::top(&output, 5).iter() {
        println!("{:50} {:5.2}%", class, 100.0 * probability)
    }
    // let image_view = ImageView::new(info, data);
    // let window = create_window("image", Default::default()).unwrap();
    // window.set_image("image-001", image_view).unwrap();
    // predict_repository::minist_predict(state);

    let mut response_body = Map::new();
    response_body.insert("score".to_string(), json!(0.5));

    let response_body = web::Json(response_body);
    Ok(HttpResponse::Ok().json(response_body))
}

fn info<P, C>(image: &image::ImageBuffer<P, C>) -> Result<ImageInfo, ImageDataError>
where
	P: image::Pixel<Subpixel = u8> + image::PixelWithColorType,
	C: std::ops::Deref<Target = [u8]>,
{
	Ok(ImageInfo {
		pixel_format: pixel_format::<P>()?,
		size: glam::UVec2::new(image.width(), image.height()),
		stride: glam::UVec2::new(
			image.sample_layout().width_stride as u32,
			image.sample_layout().height_stride as u32,
		),
	})
}

fn dynamic_image_as_bytes(image: &image::DynamicImage) -> &[u8] {
	match image {
		image::DynamicImage::ImageLuma8(x) => as_bytes(x),
		image::DynamicImage::ImageLumaA8(x) => as_bytes(x),
		image::DynamicImage::ImageLuma16(_) => panic!("unsupported pixel format: Luma16"),
		image::DynamicImage::ImageLumaA16(_) => panic!("unsupported pixel format: LumaA16"),
		image::DynamicImage::ImageRgb8(x) => as_bytes(x),
		image::DynamicImage::ImageRgba8(x) => as_bytes(x),
		image::DynamicImage::ImageRgb16(_) => panic!("unsupported pixel format: Rgb16"),
		image::DynamicImage::ImageRgba16(_) => panic!("unsupported pixel format: Rgba16"),
		image::DynamicImage::ImageRgb32F(_) => panic!("unsupported pixel format: Rgb32F"),
		image::DynamicImage::ImageRgba32F(_) => panic!("unsupported pixel format: Rgba32F"),
		x => panic!("unsupported pixel format: {:?}", x),
	}
}

fn dynamic_image_info(image: &image::DynamicImage) -> Result<ImageInfo, ImageDataError> {
	match image {
		image::DynamicImage::ImageLuma8(x) => info(x),
		image::DynamicImage::ImageLumaA8(x) => info(x),
		image::DynamicImage::ImageRgb8(x) => info(x),
		image::DynamicImage::ImageRgba8(x) => info(x),
		x => Err(UnsupportedImageFormat { format: format!("{:?}", x) }.into()),
	}
}

fn pixel_format<P: image::PixelWithColorType>() -> Result<PixelFormat, ImageDataError> {
	match P::COLOR_TYPE {
		image::ColorType::L8 => Ok(PixelFormat::Mono8),
		image::ColorType::La8 => Ok(PixelFormat::MonoAlpha8(Alpha::Unpremultiplied)),
		image::ColorType::Rgb8 => Ok(PixelFormat::Rgb8),
		image::ColorType::Rgba8 => Ok(PixelFormat::Rgba8(Alpha::Unpremultiplied)),
		x => Err(UnsupportedImageFormat { format: format!("{:?}", x) }.into()),
	}
}

fn as_bytes<P, Container>(buffer: &image::ImageBuffer<P, Container>) -> &[u8]
where
	P: image::Pixel<Subpixel = u8> + image::PixelWithColorType,
	Container: Deref<Target = [u8]>,
{
	&*buffer
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