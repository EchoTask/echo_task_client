use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::{Instant};
use chrono::Local;
use xcap::Monitor;
use tokio::time::{self, Duration};
use image::{DynamicImage, EncodableLayout, ImageBuffer, RgbaImage};
use tokio::sync::Mutex;
use webp::{Encoder, WebPMemory};
use fast_image_resize as fir;

fn normalized(filename: &str) -> String {
    filename
        .replace("|", "")
        .replace("\\", "")
        .replace(":", "")
        .replace("/", "")
}


#[tokio::main]
pub async fn start_scree_recorder() {
    let mut interval = time::interval(Duration::from_millis(1500));

    // Define a shared state for storing the last captured image.
    // Use Arc to allow safe sharing across threads, and Mutex to allow safe mutation.
    let last_image = Arc::new(Mutex::new(None::<RgbaImage>));
    loop {
        interval.tick().await;

        let last_image_clone = last_image.clone();
        tokio::spawn(async {
            get_one_record(last_image_clone).await;
        });
    }
}

async fn get_one_record(last_image: Arc<Mutex<Option<RgbaImage>>>) {
    let start = Instant::now();
    let monitors = Monitor::all().unwrap();

    for monitor in monitors {
        let image: RgbaImage = monitor.capture_image().unwrap();
        // Update the shared state with the current image for next cycle comparison
        let mut last_img = last_image.lock().await;
        *last_img = Some(image.clone());
        let resized_image = resize_image_in_stages(image.clone(), (image.width() as f32 / 1.8) as u32, (image.height() as f32 / 1.8) as u32);
        let encoder = Encoder::from_image(&resized_image).unwrap();
        let webp_data: WebPMemory = encoder.encode(90.0); // Adjust quality factor as needed
        // Format the current system time to "yyyy-mm-dd-hh-mi" format.
        let now = Local::now();
        let formatted_time = now.format("%Y-%m-%d-%H-%M-%S").to_string();

        // Generate a file name based on the monitor's name and the current system time.
        let file_name = format!("target/monitor-{}-{:?}.webp", normalized(&monitor.name()), formatted_time);
        // Save the compressed WebP data to a file.
        std::fs::write(file_name, webp_data.as_bytes()).expect("Failed to write file");
    }

    println!("Execution time: {:?}", start.elapsed());
}

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
fn resize_image_in_stages(image: RgbaImage, target_width: u32, target_height: u32) -> DynamicImage {
// Convert the RgbaImage to a raw Vec<u8> to work with fast_image_resize
    let src_buffer = image.clone().into_raw();
    let src_width = image.width();
    let src_height = image.height();

    // Prepare the source and destination dimensions
    let src_width_nz = NonZeroU32::new(src_width).expect("source width must be non-zero");
    let src_height_nz = NonZeroU32::new(src_height).expect("source height must be non-zero");
    let dst_width_nz = NonZeroU32::new(target_width).expect("destination width must be non-zero");
    let dst_height_nz = NonZeroU32::new(target_height).expect("destination height must be non-zero");

    // Create a source image view
    // Note: Adjustments may be needed here based on the specific `fast_image_resize` version and its API
    let src_image = fir::Image::from_vec_u8(src_width_nz, src_height_nz, src_buffer, fir::PixelType::U8x4)
        .expect("Failed to create source image");

    // Prepare the destination buffer
    let mut dst_buffer = vec![0u8; (target_width * target_height * 4) as usize]; // Assuming 4 bytes per pixel for RGBA

    // Create a mutable view for the destination buffer
    // Note: Adjustments may be needed here as well
    let mut dst_image = fir::Image::new(dst_width_nz, dst_height_nz, fir::PixelType::U8x4);

    // Create a resizer with a specific algorithm, for example, Lanczos3
    let mut resizer = fir::Resizer::new(fir::ResizeAlg::Convolution(fir::FilterType::Lanczos3));

    // Perform the resize operation
    resizer.resize(&src_image.view(), &mut dst_image.view_mut()).expect("Failed to resize image");

    // Convert the destination buffer back into a DynamicImage
    DynamicImage::ImageRgba8(
        ImageBuffer::from_raw(target_width, target_height, dst_image.into_vec()).expect("Failed to create image buffer from raw data")
    )
}