mod generator;
mod oauth;

use std::io::Cursor;

use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use generator::generate_sprite;
use image::{ImageOutputFormat, RgbaImage};
use oauth::{openai_callback, openai_login};

async fn generate_simple_sprite(_prompt: web::Path<String>) -> Result<HttpResponse> {
    // For simplicity, we ignore the prompt and generate a simple sprite
    let img = create_simple_sprite();

    // Convert the image to PNG format
    let mut buf: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut buf); // Create a cursor around the buffer
                                            // Use the write_to method with the cursor to write the image data into buf in PNG format
    img.write_to(&mut cursor, ImageOutputFormat::Png)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // Return the image as a response
    Ok(HttpResponse::Ok().content_type("image/png").body(buf))
}

fn create_simple_sprite() -> RgbaImage {
    // Create a simple 32x32 pixel image, filled with red pixels
    let mut img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = RgbaImage::new(32, 32);
    img.enumerate_pixels_mut().for_each(|(_, _, pixel)| {
        *pixel = image::Rgba([255, 0, 0, 255]); // Red color
    });
    img
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world! Use /generate/{prompt} to create a sprite.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/generate", web::get().to(index))
            .route("/generate/{prompt}", web::get().to(generate_simple_sprite))
            .route("/experiment/{prompt}", web::get().to(generate_sprite))
            .route("/auth/openai/login", web::get().to(openai_login))
            .route("/auth/openai/callback", web::get().to(openai_callback))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
