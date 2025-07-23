use anyhow::Result;

use crate::RagSystem;

pub async fn query(rag: &RagSystem) -> Result<()> {
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
    Ok(())
}
