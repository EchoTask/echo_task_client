use std::time::{Instant, SystemTime};
use xcap::Monitor;
use tokio::time::{self, Duration};

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

    loop {
        interval.tick().await;
        tokio::spawn(async {
            get_one_record();
        });
    }
}

fn get_one_record() {
    let start = Instant::now();
    let monitors = Monitor::all().unwrap();

    for monitor in monitors {
        let image = monitor.capture_image().unwrap();

        image
            .save(format!("target/monitor-{}-{:?}.png", normalized(monitor.name()),  SystemTime::now() ))
            .unwrap();
    }

    println!("Execution time: {:?}", start.elapsed());
}