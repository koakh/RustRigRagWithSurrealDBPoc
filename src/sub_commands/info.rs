use anyhow::Result;

use crate::RagSystem;

pub async fn info(rag: &RagSystem) -> Result<()> {
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
