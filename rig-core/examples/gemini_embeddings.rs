use rig::providers::gemini;
use rig::Embed;

#[derive(Embed, Debug)]
struct Greetings {
    id: String,
    #[embed]
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize the Google Gemini client
    // Create OpenAI client
    let client = gemini::Client::from_env();

    let embeddings = client
        .embeddings(gemini::embedding::EMBEDDING_001)
        .document(Greetings {
            id: "doc0".to_string(),
            message: "Hello, world!".to_string(),
        })?
        .document(Greetings {
            id: "doc1".to_string(),
            message: "Goodbye, world!".to_string(),
        })?
        .build()
        .await
        .expect("Failed to embed documents");

    println!("{:?}", embeddings);

    Ok(())
}