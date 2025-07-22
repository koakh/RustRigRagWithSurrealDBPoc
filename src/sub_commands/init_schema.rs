use anyhow::Result;
use tracing::info;
use std::collections::HashMap;

use crate::RagSystem;

pub async fn init_schema(rag: &RagSystem) -> Result<()> {
    // Initialize database schema
    rag.init_schema(rag.embedding_model_dimension).await?;

    // Sample documents to add to the knowledge base
    let documents = vec![
      ("Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. It was originally developed by Mozilla and is now maintained by the Rust Foundation.".to_string(), {
          let mut meta = HashMap::new();
          meta.insert("category".to_string(), "programming".to_string());
          meta.insert("language".to_string(), "rust".to_string());
          meta
      }),
      ("SurrealDB is a scalable, distributed, collaborative, document-graph database for the serverless web. It combines the flexibility of JSON documents with the power of graph queries and real-time subscriptions.".to_string(), {
          let mut meta = HashMap::new();
          meta.insert("category".to_string(), "database".to_string());
          meta.insert("type".to_string(), "document-graph".to_string());
          meta
      }),
      ("Ollama is an open-source tool that allows you to run large language models locally on your machine. It supports various models including Llama 2, Code Llama, and many others, making it easy to use AI without relying on cloud services.".to_string(), {
          let mut meta = HashMap::new();
          meta.insert("category".to_string(), "ai-tools".to_string());
          meta.insert("type".to_string(), "local-llm".to_string());
          meta
      }),
      ("Vector databases are specialized databases designed to store and query high-dimensional vectors efficiently. They are essential for semantic search, recommendation systems, and RAG applications.".to_string(), {
          let mut meta = HashMap::new();
          meta.insert("category".to_string(), "database".to_string());
          meta.insert("type".to_string(), "vector-db".to_string());
          meta
      }),
  ];

    // Store documents in the knowledge base
    let doc_ids = rag.store_documents(documents).await?;
    info!("Stored documents with IDs: {:?}", doc_ids);

    Ok(())
}
