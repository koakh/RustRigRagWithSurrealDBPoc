use anyhow::Result;
use rigrag::{Cli, Configuration, RagSystem};

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file.
    dotenvy::dotenv().expect("Failed to read .env file");

    // Init Environment Configuration
    let env_cfg = envy::prefixed("APP_")
        .from_env::<Configuration>()
        .expect("Error loading configuration...");
    println!("{:#?}", env_cfg);

    // info!("Starting RAG system with Ollama and SurrealDB");

    // Initialize RAG system
    let rag = RagSystem::new(
        &env_cfg.surreal_db_url,
        &env_cfg.surreal_db_user,
        &env_cfg.surreal_db_pass,
        &env_cfg.surreal_db_ns,
        &env_cfg.surreal_db_db,
        &env_cfg.ollama_url,
        &env_cfg.ollama_embedding_model,
        env_cfg.ollama_embedding_model_dimension,
        &env_cfg.ollama_generation_model,
    )
    .await?;

    // Initialize logging
    tracing_subscriber::fmt::init();

    // Run command Line App
    Cli::run(&rag).await;

    Ok(())
}
