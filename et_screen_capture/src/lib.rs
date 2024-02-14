use std::time::Instant;
use xcap::Monitor;

fn normalized(filename: &str) -> String {
    filename
        .replace("|", "")
        .replace("\\", "")
        .replace(":", "")
        .replace("/", "")
}

pub fn start_scree_recorder() {
    let start = Instant::now();
    let monitors = Monitor::all().unwrap();

    for monitor in monitors {

        let image = monitor.capture_image().unwrap();

        image
            .save(format!("target/monitor-{}.png", normalized(monitor.name())))
            .unwrap();
    }

    println!("Execution time: {:?}", start.elapsed());
}