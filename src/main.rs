use std::{env, sync::Arc};

use audio::RECORDING_PATH;
use axum::{
    extract::State,
    routing::{get, post},
    Router,
};
use openai::Client;
use transcription::transcribe_file;

mod audio;
mod openai;
mod transcription;

pub struct AppState {
    pub openai_client: Client,
}

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");
    let client = Client::new(api_key);

    let app = Router::new()
        .route("/_status", get(status))
        .route("/record", post(record))
        .route("/test/transcribe", post(transcribe_existing_file))
        .with_state(Arc::new(AppState {
            openai_client: client,
        }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4004").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn status() -> &'static str {
    "OK"
}

async fn record() -> &'static str {
    // ideally triggers recording and returns ack early
    audio::start_recording().expect("success?");

    "Started recording"
}

async fn transcribe_existing_file(State(state): State<Arc<AppState>>) -> String {
    // eagerly transcribes existing file, use for tests
    transcribe_file(&state.openai_client, RECORDING_PATH)
        .await
        .unwrap()
}
