use actix_web::Result;

pub async fn convert_image_to_gif(image_data: Vec<u8>) -> Result<Vec<u8>> {
    // Use the image crate to convert the image format
    // This requires reading the input image data, then saving it as GIF
    todo!()
}