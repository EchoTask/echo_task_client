use std::num::NonZeroU32;
use image::{DynamicImage, ImageBuffer, RgbaImage};
use fast_image_resize as fir;

/// Resizes an image in stages for improved performance.
///
/// # Arguments
///
/// * `image` - The source `RgbaImage`.
/// * `target_width` - The target width of the resized image.
/// * `target_height` - The target height of the resized image.
///
/// # Returns
///
/// A `DynamicImage` resized to the target dimensions.
pub fn resize_image_img(image: RgbaImage) -> DynamicImage {
    // Convert the RgbaImage to a raw Vec<u8> to work with fast_image_resize

    let src_width = NonZeroU32::new(image.width()).expect("Img Resize Error: source width must be non-zero");
    let src_height = NonZeroU32::new(image.height()).expect("Img Resize Error: source height must be non-zero");
    let target_size = resize_image_with_max_width(image.width(), image.height());
    let src_buffer = image.into_raw(); // Cloning removed to avoid unnecessary duplication

    // Create a source image
    let src_image = fir::Image::from_vec_u8(src_width, src_height, src_buffer, fir::PixelType::U8x4)
        .expect("Img Resize Error: Failed to create source image");


    // Destination dimensions
    let dst_width_nz = NonZeroU32::new(target_size.0).expect("Img Resize Error: destination width must be non-zero");
    let dst_height_nz = NonZeroU32::new(target_size.1).expect("Img Resize Error: destination height must be non-zero");

    // Create a destination image
    let mut dst_image = fir::Image::new(dst_width_nz, dst_height_nz, fir::PixelType::U8x4);

    // Create a resizer with a specific algorithm, e.g., Lanczos3
    let mut resizer = fir::Resizer::new(fir::ResizeAlg::Convolution(fir::FilterType::Lanczos3));

    // Perform the resize operation
    resizer.resize(&src_image.view(), &mut dst_image.view_mut()).expect("Img Resize Error: Failed to resize image");

    // Convert the destination buffer back into a DynamicImage
    DynamicImage::ImageRgba8(
        ImageBuffer::from_raw(target_size.0, target_size.1, dst_image.into_vec()).expect("Img Resize Error: Failed to create image buffer from raw data")
    )
}


pub fn resize_image_with_max_width(original_width: u32, original_height: u32) -> (u32, u32) {

    // Calculate provisional target_width based on the scaling factor of 1.8
    let provisional_target_width = (original_width as f32 / 1.7) as u32;

    // Ensure target_width does not exceed 1080
    let target_width = provisional_target_width.max(1080);

    // Calculate target_height to maintain the aspect ratio
    // Use the actual scaling factor applied to the width to calculate the height
    let scaling_factor = target_width as f32 / original_width as f32;
    let target_height = (original_height as f32 * scaling_factor) as u32;

    (target_width, target_height)
}