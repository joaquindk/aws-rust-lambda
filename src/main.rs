use env_logger;
use http::StatusCode;
use log::{debug, info};
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter, Rejection};

#[tokio::main]
async fn main() {
    env_logger::init();
    let server_port = 8080;
    let log = warp::log("warp_server");

    let hello_path = warp::path::end().and(warp::get()).map(|| {
        debug!("Received incoming request!");
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(String::from("Hello Ditto!!"))
    });
    info!("Starting portal server on port: {}", server_port);

    let routes = hello_path
        .recover(|err: Rejection| async move {
            debug!("Failed request!");
            return Ok::<_, Infallible>(warp::reply::with_status(
                format!("Error: {:?}", err),
                StatusCode::BAD_REQUEST,
            ));
        })
        .with(log);

    warp::serve(routes).run(([0, 0, 0, 0], server_port)).await
}
