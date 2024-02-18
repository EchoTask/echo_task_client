mod img_resizer;
mod img_different_cheker;

use std::sync::Arc;
use std::time::Instant;
use chrono::Local;
use xcap::Monitor;
use tokio::time::Duration;
use image::{EncodableLayout, RgbaImage};
use tokio::sync::Mutex;
use webp::{Encoder, WebPMemory};
use tracing::{info, warn, error};
use crate::img_different_cheker::is_image_different_enough;

fn normalized(filename: &str) -> String {
    filename
        .replace("|", "")
        .replace("\\", "")
        .replace(":", "")
        .replace("/", "")
}

#[tokio::main]
pub async fn start_screen_recorder() {
    let last_image = Arc::new(Mutex::new(None::<RgbaImage>));

    loop {
        tokio::time::sleep(Duration::from_millis(2000)).await;
        let last_image_clone = last_image.clone();
        tokio::spawn(async move {
            if let Err(e) = get_one_record(last_image_clone).await {
                error!("Failed to get one record: {}", e);
            }
        });
    }
}

async fn get_one_record(last_image: Arc<Mutex<Option<RgbaImage>>>) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let monitors = Monitor::all()?;

    for monitor in monitors {
        let image: RgbaImage = monitor.capture_image()?;
        let mut last_img = last_image.lock().await;

        if let Some(ref last_image) = *last_img {
            if is_image_different_enough(&image, last_image) {
                info!("Current image is significantly different from the last image.");
                // Update the last image after successful processing
                *last_img = Some(image.clone());

                // Directly pass the image to the function without dereferencing the MutexGuard
                run_save_new_img(monitor, image).await?;
            } else {
                info!("Current image is not significantly different from the last image. Skipping.");
                continue; // Skip this image
            }
        } else {
            info!("No last image available for comparison.");
            // As this is the first image, save it and update the last image
            *last_img = Some(image.clone());
            run_save_new_img(monitor, image).await?;
        }
    }

    info!("Execution time: {:?}", start.elapsed());
    Ok(())
}

async fn run_save_new_img(monitor: Monitor, new_img: RgbaImage) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(resized_image) = img_resizer::resize_image_img(new_img) {
        let encoder: Encoder = Encoder::from_image(&resized_image)?;
        let webp_data: WebPMemory = encoder.encode(90.0); // Quality factor
        let now = Local::now();
        let formatted_time: String = now.format("%Y-%m-%d-%H-%M-%S").to_string();
        let file_name: String = format!("target/monitor-{}-{:?}.webp", normalized(&monitor.name()), formatted_time);
        std::fs::write(file_name, webp_data.as_bytes())?;
    } else {
        warn!("Failed to resize image for monitor '{}'", monitor.name());
    }

    Ok(())
}