use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use surrealdb::sql::Thing;

// Data structures for Ollama API
#[derive(Debug, Deserialize)]
pub struct OllamaEmbeddingResponse {
  pub embedding: Vec<f32>,
}

#[derive(Debug, Deserialize)]
pub struct OllamaGenerationResponse {
  pub response: String,
}

// Document structure for our RAG system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: Thing,
    pub content: String,
    pub metadata: HashMap<String, String>,
    pub embedding: Vec<f32>,
    pub created_at: String,
}