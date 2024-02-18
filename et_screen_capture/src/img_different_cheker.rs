use image_hasher::HasherConfig;
use tracing::{info, debug};

// Function to check if two images are not the same for at least 95% similarity
pub fn is_image_different_enough(current_image: &image::RgbaImage, last_image: &image::RgbaImage) -> bool {
    let dynamic_current_image = image::DynamicImage::ImageRgba8(current_image.clone());
    let dynamic_last_image = image::DynamicImage::ImageRgba8(last_image.clone());

    let hasher = HasherConfig::new().to_hasher();

    let hash_current = hasher.hash_image(&dynamic_current_image);
    let hash_last = hasher.hash_image(&dynamic_last_image);


    // Calculate the distance between the two hashes. A smaller distance means the images are more similar.
    let distance = hash_current.dist(&hash_last);

    debug!("Hash Distance: {}", distance);

    if distance > 0 {
        info!("Images are considered different enough. Proceeding with further processing.");
        true
    } else {
        info!("Images are not considered different enough. Skipping further processing.");
        false
    }
}

