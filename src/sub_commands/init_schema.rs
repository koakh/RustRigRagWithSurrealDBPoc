use anyhow::Result;

use crate::RagSystem;

pub async fn init_schema(rag: &RagSystem) -> Result<()> {
    // Initialize database schema
    rag.init_schema(rag.embedding_model_dimension).await?;

    Ok(())
}
