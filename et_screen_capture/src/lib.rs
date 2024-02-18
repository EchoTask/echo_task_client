mod img_resizer;

use std::sync::Arc;
use std::thread::sleep;
use std::time::Instant;
use chrono::Local;
use xcap::Monitor;
use tokio::time::{self, Duration};
use image::{EncodableLayout, RgbaImage};
use tokio::sync::Mutex;
use webp::Encoder;
use tracing::{info, warn, error};

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
        let image = monitor.capture_image()?;
        let mut last_img = last_image.lock().await;
        *last_img = Some(image.clone());

        if let Some(resized_image) = img_resizer::resize_image_img(image) {
            let encoder = Encoder::from_image(&resized_image)?;
            let webp_data = encoder.encode(90.0); // Quality factor
            let now = Local::now();
            let formatted_time = now.format("%Y-%m-%d-%H-%M-%S").to_string();
            let file_name = format!("target/monitor-{}-{:?}.webp", normalized(&monitor.name()), formatted_time);
            std::fs::write(file_name, webp_data.as_bytes())?;
        } else {
            warn!("Failed to resize image for monitor '{}'", monitor.name());
        }
    }

    info!("Execution time: {:?}", start.elapsed());
    Ok(())
}
