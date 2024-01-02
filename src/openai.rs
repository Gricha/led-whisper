// current implementations of openai are very unreliable, instead of wasting more time im just gonna
// write a few methods i need myself *shrug*

use std::{fs::File, io::Read, path::Path};

use anyhow::anyhow;
use reqwest::Method;

pub struct Client {
    api_key: String,
}

pub struct AudioTranscriptionParameters {
    pub path: String,
    pub model: String,
    pub prompt: Option<String>,
    pub language: Option<String>,
    pub response_format: Option<AudioResponseFormat>,
    pub temperature: Option<f64>,
}

pub enum AudioResponseFormat {
    Json,
    Text,
    Srt,
    VerboseJson,
    Vtt,
}

impl ToString for AudioResponseFormat {
    fn to_string(&self) -> String {
        match self {
            AudioResponseFormat::Json => "json",
            AudioResponseFormat::Text => "text",
            AudioResponseFormat::Srt => "srt",
            AudioResponseFormat::VerboseJson => "verbose_json",
            AudioResponseFormat::Vtt => "vtt",
        }
        .to_string()
    }
}

impl Client {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub async fn audio_transcribe(
        &self,
        parameters: AudioTranscriptionParameters,
    ) -> Result<String, anyhow::Error> {
        let mut form = reqwest::multipart::Form::new();

        form = form.part("file", load_file_into_part_stream(&parameters.path)?);

        form = form.text("model", parameters.model);

        if let Some(prompt) = parameters.prompt {
            form = form.text("prompt", prompt);
        }

        if let Some(language) = parameters.language {
            form = form.text("language", language.to_string());
        }

        if let Some(response_format) = parameters.response_format {
            form = form.text("response_format", response_format.to_string());
        }

        if let Some(temperature) = parameters.temperature {
            form = form.text("temperature", temperature.to_string());
        }

        let client = reqwest::Client::new();
        let response = client
            .request(
                Method::POST,
                "https://api.openai.com/v1/audio/transcriptions",
            )
            .bearer_auth(&self.api_key)
            .multipart(form)
            .send()
            .await
            .unwrap();

        if response.status().is_server_error() {
            return Err(anyhow!(response.text().await.unwrap()));
        }

        Ok(response.text().await.unwrap())
    }
}

// helper function, will load up an audio file into a multiform part and parse out metadata for it
fn load_file_into_part_stream(path: &str) -> Result<reqwest::multipart::Part, anyhow::Error> {
    let mut file = File::open(path)?;
    let mut file_content = Vec::new();
    file.read_to_end(&mut file_content)?;

    let path = Path::new(path);
    let file_name = path
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("Invalid file name"))?
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Non UTF-8 file name"))?;

    let mime_type = mime_guess::from_path(path)
        .first_or_octet_stream()
        .to_string();

    let file_part = reqwest::multipart::Part::stream(file_content)
        .file_name(file_name.to_owned())
        .mime_str(&mime_type)?;
    Ok(file_part)
}
