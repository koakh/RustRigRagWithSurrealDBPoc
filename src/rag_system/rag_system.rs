use anyhow::Result;
use reqwest::Client;
use serde_json::json;
use surrealdb::engine::remote::ws::{ Client as WsClient, Ws };
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use tracing::{ info, warn };
use uuid::Uuid;
use crate::rag_system::structures::Document;

use super::structures::{ DocumentMetaData, OllamaEmbeddingResponse, OllamaGenerationResponse };

// RAG System
pub struct RagSystem {
    db: Surreal<WsClient>,
    pub ollama_http_client: Client,
    ollama_url: String,
    embedding_model: String,
    pub embedding_model_dimension: u16,
    generation_model: String,
}

impl RagSystem {
    pub async fn new(
        db_url: &str,
        db_pass: &str,
        db_user: &str,
        db_ns: &str,
        db_db: &str,
        ollama_url: &str,
        embedding_model: &str,
        embedding_model_dimension: u16,
        generation_model: &str
    ) -> Result<Self> {
        // Connect to SurrealDB
        let db = Surreal::new::<Ws>(db_url).await?;
        db.signin(Root {
            username: db_user,
            password: db_pass,
        }).await?;
        db.use_ns(db_ns.to_owned()).use_db(db_db.to_owned()).await?;

        // Create HTTP client for Ollama
        let ollama_http_client = Client::new();

        info!("RAG System initialized successfully");

        Ok(RagSystem {
            db,
            ollama_http_client,
            ollama_url: ollama_url.to_string(),
            embedding_model: embedding_model.to_string(),
            generation_model: generation_model.to_string(),
            embedding_model_dimension,
        })
    }

    // Initialize database schema
    pub async fn init_schema(&self, embedding_model_dimension: u16) -> Result<()> {
        // Create documents table with vector index
        self.db.query(
            format!("
                DEFINE TABLE documents SCHEMAFULL;
                DEFINE FIELD id ON documents TYPE string;
                DEFINE FIELD content ON documents TYPE string;
                DEFINE FIELD embedding ON documents TYPE array<float>;
                DEFINE FIELD metadata ON documents TYPE object;
                DEFINE FIELD created_at ON documents TYPE string;
                DEFINE INDEX embedding_idx ON documents FIELDS embedding MTREE DIMENSION {};
                ", embedding_model_dimension)
        ).await?;

        info!("Database schema initialized with dimensions: {}", embedding_model_dimension);
        Ok(())
    }

    // Generate embedding using Ollama
    pub async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        let response = self.ollama_http_client
            .post(&format!("{}/api/embeddings", self.ollama_url))
            .json(
                &json!({
                "model": self.embedding_model,
                "prompt": text
            })
            )
            .send().await?;

        let embedding: OllamaEmbeddingResponse = response.json().await?;
        Ok(embedding.embedding)
    }

    // Store document with embedding
    pub async fn store_document(
        &self,
        content: &str,
        metadata: DocumentMetaData
    ) -> Result<String> {
        let embedding = self.generate_embedding(content).await?;
        let doc_id = Uuid::new_v4().to_string();
        let record_id = ("documents", &doc_id);

        let doc = Document {
            id: Thing::from(("documents", doc_id.as_str())),
            content: content.to_string(),
            embedding,
            metadata,
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        // info!("doc.metadata: {:?}", &doc.metadata.clone());

        let created_doc: Document = self.db
            .create(record_id)
            .content(doc).await?
            .ok_or_else(|| anyhow::anyhow!("Failed to create document"))?;

        Ok(created_doc.id.to_string())
    }

    // Store multiple documents
    pub async fn store_documents(
        &self,
        documents: Vec<(String, DocumentMetaData)>
    ) -> Result<Vec<String>> {
        let mut doc_ids = Vec::new();

        for (content, metadata) in documents {
            // info!("metadata: {:?}", &metadata.clone());
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
        let results: Vec<Document> = self.db
            .query(
                "
                SELECT * FROM documents
                WHERE vector::similarity::cosine(embedding, $embedding) > 0.5
                ORDER BY similarity DESC
                LIMIT $limit
                "
            )
            .bind(("embedding", query_embedding))
            .bind(("limit", limit)).await?
            .take(0)?;

        info!("Retrieved {} similar documents", results.len());
        Ok(results)
    }

    // Generate response using retrieved context
    pub async fn generate_response(
        &self,
        query: &str,
        context_docs: &[Document]
    ) -> Result<String> {
        let context = context_docs
            .iter()
            .map(|doc| doc.content.clone())
            .collect::<Vec<_>>()
            .join("\n\n");

        let prompt = format!(
            "Context:\n{}\n\nQuestion: {}\n\nAnswer based on the context above. If the context doesn't contain enough information, say so:",
            context,
            query
        );

        let response = self.ollama_http_client
            .post(&format!("{}/api/generate", self.ollama_url))
            .json(
                &json!({
                "model": self.generation_model,
                "prompt": prompt,
                "stream": false
            })
            )
            .send().await?;

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
