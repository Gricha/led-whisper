use axum::{
    routing::{get, post},
    Router,
};
mod audio;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/_status", get(status))
        .route("/record", post(record));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn status() -> &'static str {
    "OK"
}

async fn record() -> &'static str {
    // ideally triggers recording and returns ack early

    audio::start_recording().await.expect("success?");

    "Started recording"
}
