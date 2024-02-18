use std::num::NonZeroU32;
use image::{DynamicImage, ImageBuffer, RgbaImage};
use fast_image_resize as fir;
use tracing::{info, error, instrument};

/// Resizes an image in stages for improved performance.
///
/// # Arguments
///
/// * `image` - The source `RgbaImage`.
///
/// # Returns
///
/// An `Option<DynamicImage>` resized to the target dimensions or `None` if the operation fails.
pub fn resize_image_img(image: RgbaImage) -> Option<DynamicImage> {
    // info!("Starting image resize");

    let src_width = NonZeroU32::new(image.width());
    let src_height = NonZeroU32::new(image.height());

    if src_width.is_none() || src_height.is_none() {
        error!("Img Resize Error: source width or height is zero");
        return None;
    }

    let target_size = resize_image_with_max_width(image.width(), image.height());
    info!("Target resize dimensions: {:?}", target_size);

    let src_buffer = image.into_raw();

    let src_image = fir::Image::from_vec_u8(src_width.unwrap(), src_height.unwrap(), src_buffer, fir::PixelType::U8x4)
        .ok();

    if src_image.is_none() {
        error!("Img Resize Error: Failed to create source image");
        return None;
    }

    let dst_width_nz = NonZeroU32::new(target_size.0);
    let dst_height_nz = NonZeroU32::new(target_size.1);

    if dst_width_nz.is_none() || dst_height_nz.is_none() {
        error!("Img Resize Error: destination width or height is zero");
        return None;
    }

    let mut dst_image = fir::Image::new(dst_width_nz.unwrap(), dst_height_nz.unwrap(), fir::PixelType::U8x4);
    info!("Destination image initialized");

    let mut resizer = fir::Resizer::new(fir::ResizeAlg::Convolution(fir::FilterType::Lanczos3));
    info!("Resizer created with Lanczos3 algorithm");

    if resizer.resize(&src_image.unwrap().view(), &mut dst_image.view_mut()).is_err() {
        error!("Img Resize Error: Failed to resize image");
        return None;
    }
    info!("Image resizing completed");

    // Handle potential failure from `ImageBuffer::from_raw` gracefully
    match ImageBuffer::from_raw(target_size.0, target_size.1, dst_image.into_vec()) {
        Some(buffer) => Some(DynamicImage::ImageRgba8(buffer)),
        None => {
            error!("Img Resize Error: Failed to create image buffer from raw data");
            None
        }
    }
}

pub fn resize_image_with_max_width(original_width: u32, original_height: u32) -> (u32, u32) {
    info!("Calculating target dimensions with max width strategy");

    let provisional_target_width = (original_width as f32 / 1.7) as u32;
    let target_width = provisional_target_width.max(1080);
    let scaling_factor = target_width as f32 / original_width as f32;
    let target_height = (original_height as f32 * scaling_factor) as u32;

    info!("Target dimensions calculated: width={}, height={}", target_width, target_height);

    (target_width, target_height)
}
