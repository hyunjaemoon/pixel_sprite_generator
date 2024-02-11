mod gif_convert;
mod image_gen_api;

use actix_web::{web, HttpResponse, Result};

use self::{gif_convert::convert_image_to_gif, image_gen_api::call_ai_image_generation_api};

pub async fn generate_sprite(prompt: web::Path<String>) -> Result<HttpResponse> {
    // Step 1: Send prompt to an image generation API
    let api_response = call_ai_image_generation_api(&prompt).await?;

    // Step 2: Convert the received image to GIF format
    let gif_image = convert_image_to_gif(api_response).await?;

    // Step 3: Return the GIF image in the response
    Ok(HttpResponse::Ok().content_type("image/gif").body(gif_image))
}
