use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    et_logger::init_logger();

    // et_screen_capture::start_screen_recorder();
    et_database::run_database_connection().await;
}