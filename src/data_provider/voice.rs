use crate::dictionary::app_data_dir;
use rodio::Decoder;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::BufReader;
use reqwest::Client;

pub async fn get_voice(text: &str) -> BufReader<File> {
    let mut path = app_data_dir();
    path.push("voice");
    if !path.exists() {
        std::fs::create_dir(path.clone()).unwrap();
    }
    let hash = format!("{}.wav", hex::encode(Sha256::digest(text.as_bytes())));
    path.push(hash);
    if !path.exists() {
        let engine_url = "http://127.0.0.1:50021";
        let client = Client::new();
        let query = client
            .post(format!("{engine_url}/audio_query?text={text}&speaker=11"))
            .send()
            .await;

        let query = query.unwrap().text().await.unwrap();
        // Synthesis
        let audio = client
            .post(format!("{engine_url}/synthesis?speaker=11"))
            .header("Content-Type", "application/json")
            .body(query)
            .send()
            .await.unwrap()
            .bytes()
            .await.unwrap();

        tokio::fs::write(&path, &audio).await.unwrap();
    }

    BufReader::new(File::open(path).unwrap())
}
