use crate::openai::{AudioResponseFormat, AudioTranscriptionParameters, Client};

pub async fn transcribe_file(client: &Client, path: &str) -> Result<String, anyhow::Error> {
    println!("Transcribing file at {path}");

    let parameters = AudioTranscriptionParameters {
        path: path.to_string(),
        model: "whisper-1".to_string(),
        language: None,
        prompt: None,
        response_format: Some(AudioResponseFormat::Text),
        temperature: None,
    };

    let result = client.audio_transcribe(parameters).await?;

    println!("Transcribed successfully. Response is:\n{result}");

    Ok(result)
}
