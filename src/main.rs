use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use surrealdb::engine::remote::ws::{Client as WsClient, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tracing::{info, warn};
use uuid::Uuid;

// Data structures for Ollama API
#[derive(Debug, Deserialize)]
struct OllamaEmbeddingResponse {
    embedding: Vec<f32>,
}

#[derive(Debug, Deserialize)]
struct OllamaGenerationResponse {
    response: String,
}

// Document structure for our RAG system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub content: String,
    pub metadata: HashMap<String, String>,
    pub embedding: Vec<f32>,
    pub created_at: String,
}

// RAG System
pub struct RagSystem {
    db: Surreal<WsClient>,
    ollama_client: Client,
    ollama_url: String,
    embedding_model: String,
    generation_model: String,
}

impl RagSystem {
    pub async fn new(
        db_url: &str,
        ollama_url: &str,
        embedding_model: &str,
        generation_model: &str,
    ) -> Result<Self> {
        // Connect to SurrealDB
        let db = Surreal::new::<Ws>(db_url).await?;
        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await?;
        db.use_ns("rag").use_db("documents").await?;

        // Create HTTP client for Ollama
        let ollama_client = Client::new();

        info!("RAG System initialized successfully");

        Ok(RagSystem {
            db,
            ollama_client,
            ollama_url: ollama_url.to_string(),
            embedding_model: embedding_model.to_string(),
            generation_model: generation_model.to_string(),
        })
    }

    // Initialize database schema
    pub async fn init_schema(&self) -> Result<()> {
        // Create documents table with vector index
        self.db
            .query(
                "
                DEFINE TABLE documents SCHEMAFULL;
                DEFINE FIELD id ON documents TYPE string;
                DEFINE FIELD content ON documents TYPE string;
                DEFINE FIELD embedding ON documents TYPE array<float>;
                DEFINE FIELD metadata ON documents TYPE object;
                DEFINE FIELD created_at ON documents TYPE string;
                DEFINE INDEX embedding_idx ON documents FIELDS embedding MTREE DIMENSION 384;
                ",
            )
            .await?;

        info!("Database schema initialized");
        Ok(())
    }

    // Generate embedding using Ollama
    pub async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        let response = self
            .ollama_client
            .post(&format!("{}/api/embeddings", self.ollama_url))
            .json(&json!({
                "model": self.embedding_model,
                "prompt": text
            }))
            .send()
            .await?;

        let embedding: OllamaEmbeddingResponse = response.json().await?;
        Ok(embedding.embedding)
    }

    // Store document with embedding
    pub async fn store_document(
        &self,
        content: &str,
        metadata: HashMap<String, String>,
    ) -> Result<String> {
        let embedding = self.generate_embedding(content).await?;
        let doc_id = Uuid::new_v4().to_string();

        let doc = Document {
            id: doc_id.clone(),
            content: content.to_string(),
            embedding,
            metadata,
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        let _documents: Vec<Document> = self
            .db
            .create("documents")
            .content(doc)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Failed to create document"))?;

        info!("Document stored successfully with ID: {}", doc_id);
        Ok(doc_id)
    }

    // Store multiple documents
    pub async fn store_documents(
        &self,
        documents: Vec<(String, HashMap<String, String>)>,
    ) -> Result<Vec<String>> {
        let mut doc_ids = Vec::new();

        for (content, metadata) in documents {
            let doc_id = self.store_document(&content, metadata).await?;
            doc_ids.push(doc_id);
        }

        info!("Stored {} documents", doc_ids.len());
        Ok(doc_ids)
    }

    // Retrieve similar documents
    pub async fn retrieve_similar(&self, query: &str, limit: usize) -> Result<Vec<Document>> {
        let query_embedding = self.generate_embedding(query).await?;

        // Using vector similarity search (cosine similarity)
        let results: Vec<Document> = self
            .db
            .query(
                "
                SELECT * FROM documents
                WHERE vector::similarity::cosine(embedding, $embedding) > 0.5
                ORDER BY vector::similarity::cosine(embedding, $embedding) DESC
                LIMIT $limit
                ",
            )
            .bind(("embedding", query_embedding))
            .bind(("limit", limit))
            .await?
            .take(0)?;

        info!("Retrieved {} similar documents", results.len());
        Ok(results)
    }

    // Generate response using retrieved context
    pub async fn generate_response(
        &self,
        query: &str,
        context_docs: &[Document],
    ) -> Result<String> {
        let context = context_docs
            .iter()
            .map(|doc| doc.content.clone())
            .collect::<Vec<_>>()
            .join("\n\n");

        let prompt = format!(
            "Context:\n{}\n\nQuestion: {}\n\nAnswer based on the context above. If the context doesn't contain enough information, say so:",
            context, query
        );

        let response = self
            .ollama_client
            .post(&format!("{}/api/generate", self.ollama_url))
            .json(&json!({
                "model": self.generation_model,
                "prompt": prompt,
                "stream": false
            }))
            .send()
            .await?;

        let generation: OllamaGenerationResponse = response.json().await?;
        Ok(generation.response)
    }

    // Complete RAG pipeline
    pub async fn query(&self, question: &str) -> Result<String> {
        info!("Processing query: {}", question);

        // Step 1: Retrieve similar documents
        let similar_docs = self.retrieve_similar(question, 5).await?;

        if similar_docs.is_empty() {
            warn!("No relevant documents found in the knowledge base");
            return Ok("No relevant documents found in the knowledge base.".to_string());
        }

        // Step 2: Generate response using retrieved context
        let response = self.generate_response(question, &similar_docs).await?;

        info!("Generated response for query");
        Ok(response)
    }

    // Get document by ID
    pub async fn get_document(&self, doc_id: &str) -> Result<Option<Document>> {
        let result: Option<Document> = self.db.select(("documents", doc_id)).await?;

        Ok(result)
    }

    // List all documents
    pub async fn list_documents(&self) -> Result<Vec<Document>> {
        let documents: Vec<Document> = self.db.select("documents").await?;

        Ok(documents)
    }

    // Delete document
    pub async fn delete_document(&self, doc_id: &str) -> Result<()> {
        let _: Option<Document> = self.db.delete(("documents", doc_id)).await?;

        info!("Document deleted: {}", doc_id);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("Starting RAG system with Ollama and SurrealDB");

    // Initialize RAG system
    let rag = RagSystem::new(
        "127.0.0.1:8000",         // SurrealDB URL
        "http://localhost:11434", // Ollama URL
        "nomic-embed-text",       // Embedding model
        "llama3.2",               // Generation model
    )
    .await?;

    // Initialize database schema
    rag.init_schema().await?;

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
        println!("- Document ID: {}", doc.id);
        println!(
            "  Content preview: {}...",
            doc.content.chars().take(100).collect::<String>()
        );
        println!("  Metadata: {:?}", doc.metadata);
        println!();
    }

    Ok(())
}
