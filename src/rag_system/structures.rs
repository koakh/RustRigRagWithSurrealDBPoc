use serde::{ Deserialize, Serialize };
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetaData {
    pub index: usize,
    pub source: String,
}

// Document structure for our RAG system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: Thing,
    pub content: String,
    pub metadata: DocumentMetaData,
    pub embedding: Vec<f32>,
    pub created_at: String,
}
