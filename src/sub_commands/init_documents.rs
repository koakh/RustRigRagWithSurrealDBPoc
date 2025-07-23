use anyhow::Result;
use tracing::info;
use crate::{ rag_system::DocumentMetaData, RagSystem };

pub async fn init_documents(rag: &RagSystem) -> Result<()> {
    // Sample documents to add to the knowledge base
    let documents = vec![
        (
            "Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. It was originally developed by Mozilla and is now maintained by the Rust Foundation.".to_string(),
            DocumentMetaData { index: 0, source: "moke1.txt ".to_string() },
        ),
        (
            "SurrealDB is a scalable, distributed, collaborative, document-graph database for the serverless web. It combines the flexibility of JSON documents with the power of graph queries and real-time subscriptions.".to_string(),
            DocumentMetaData { index: 0, source: "moke2.txt ".to_string() },
        ),
        (
            "Ollama is an open-source tool that allows you to run large language models locally on your machine. It supports various models including Llama 2, Code Llama, and many others, making it easy to use AI without relying on cloud services.".to_string(),
            DocumentMetaData { index: 0, source: "moke3.txt ".to_string() },
        ),
        (
            "Vector databases are specialized databases designed to store and query high-dimensional vectors efficiently. They are essential for semantic search, recommendation systems, and RAG applications.".to_string(),
            DocumentMetaData { index: 0, source: "moke4.txt ".to_string() },
        )
    ];

    // Store documents in the knowledge base
    let doc_ids = rag.store_documents(documents).await?;
    info!("Stored documents with IDs: {:?}", doc_ids);

    Ok(())
}
