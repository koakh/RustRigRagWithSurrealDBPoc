use anyhow::Result;
use rag_system::{Cli, Configuration, RagSystem};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use surrealdb::engine::remote::ws::{Client as WsClient, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use tracing::{info, warn};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file.
    dotenvy::dotenv().expect("Failed to read .env file");
    // Init Environment Configuration

    // let env_cfg: Configuration = from_env().expect("Error loading configuration...");
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
        &env_cfg.ollama_generation_model,
    )
    .await?;

    // Initialize logging
    tracing_subscriber::fmt::init();

    // Run command Line App
    Cli::run(&rag).await;

    todo!();


    // Example queries
    let queries = vec![
        "What is Rust programming language?",
        "How does SurrealDB work?",
        "What is Ollama used for?",
        "What are vector databases?",
        "How can I run LLMs locally?",
    ];

    // Query the RAG system
    for query in queries {
        println!("\n{}", "=".repeat(60));
        println!("Question: {}", query);

        match rag.query(query).await {
            Ok(answer) => println!("Answer: {}", answer),
            Err(e) => println!("Error: {}", e),
        }
    }

    // Show some system info
    println!("\n{}", "=".repeat(60));
    println!("System Information:");

    let all_docs = rag.list_documents().await?;
    println!("Total documents in knowledge base: {}", all_docs.len());

    for doc in all_docs {
        println!("- Document ID: {}", doc.id.id);
        println!(
            "  Content preview: {}...",
            doc.content.chars().take(100).collect::<String>()
        );
        println!("  Metadata: {:?}", doc.metadata);
        println!();
    }

    Ok(())
}
