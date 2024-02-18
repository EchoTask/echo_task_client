mod img_resizer;

use std::sync::Arc;
use std::time::{Instant};
use chrono::Local;
use xcap::Monitor;
use tokio::time::{self, Duration};
use image::{EncodableLayout, RgbaImage};
use tokio::sync::Mutex;
use webp::{Encoder, WebPMemory};

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
        let resized_image = img_resizer::resize_image_img(image);
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

