extern crate pretty_env_logger;
use warp::Filter;

#[macro_use] extern crate log;
#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    info!("Starting Picard!");
    warp::serve(warp::fs::dir("public_html"))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
