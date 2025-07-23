use anyhow::{ Context, Result };
use rig::loaders::PdfFileLoader;
use std::path::PathBuf;
use std::collections::HashMap;
use tracing::info;
use std::fs;

use crate::{ rag_system::DocumentMetaData, sanitize_chunk_comprehensive, RagSystem };

fn load_pdf(path: PathBuf) -> Result<Vec<String>> {
    let mut chunks = Vec::new();
    let mut current_chunk = String::new();
    // TODO: move to constants
    let chunk_size = 2000; // Approximately 2000 characters per chunk

    for entry in PdfFileLoader::with_glob(path.to_str().unwrap())?.read() {
        let content = entry?;

        // Split content into words
        let words: Vec<&str> = content.split_whitespace().collect();

        for word in words {
            if current_chunk.len() + word.len() + 1 > chunk_size {
                // If adding the next word would exceed chunk size,
                // save current chunk and start a new one
                if !current_chunk.is_empty() {
                    chunks.push(current_chunk.trim().to_string());
                    current_chunk.clear();
                }
            }
            current_chunk.push_str(word);
            current_chunk.push(' ');
        }
    }

    // last chunk
    if !current_chunk.is_empty() {
        chunks.push(current_chunk.trim().to_string());
    }

    if chunks.is_empty() {
        anyhow::bail!("No content found in PDF file: {:?}", path);
    }

    Ok(chunks)
}

pub async fn init_pdf_documents(rag: &RagSystem) -> Result<()> {
    let mut documents = Vec::new();
    let moores_law_for_everything_pdf_file_path = "Moores_Law_for_Everything.pdf";
    let the_last_question_pdf_file_path = "The_Last_Question.pdf";

    // Load PDFs using Rig's built-in PDF loader
    let documents_dir = std::env::current_dir()?.join("documents");

    let paths = fs::read_dir("./documents").unwrap();
    for path in paths {
        match path {
            Ok(p) => {
                let source = p.file_name().to_string_lossy().to_string();
                info!("file path: {}", p.path().display());
                let document_vector = load_pdf(p.path().into()).context(
                    "Failed to load Moores_Law_for_Everything.pdf"
                )?;
                info!("chunking source: {}", source);
                for (i, chunk) in document_vector.into_iter().enumerate() {
                    // required to sanitize to prevent server crash with `NUL bytes (\0) in your PDF text chunks`
                    let sanitized_chunk = sanitize_chunk_comprehensive(chunk.as_str());
                    // info!("i: {}, chunk: {}", i, sanitized_chunk);
                    documents.push((
                        sanitized_chunk,
                        DocumentMetaData {
                            index: i,
                            source: p.file_name().to_string_lossy().to_string(),
                        },
                    ));
                }
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    // let moores_law_chunks = load_pdf(
    //     documents_dir.join(moores_law_for_everything_pdf_file_path)
    // ).context("Failed to load Moores_Law_for_Everything.pdf")?;
    // let last_question_chunks = load_pdf(
    //     documents_dir.join(the_last_question_pdf_file_path)
    // ).context("Failed to load The_Last_Question.pdf")?;

    info!("Successfully loaded and chunked PDF documents");

    // Add chunks from Moore's Law
    // info!("chunking Moore's Law");
    // for (i, chunk) in moores_law_chunks.into_iter().enumerate() {
    //     // required to sanitize to prevent server crash with `NUL bytes (\0) in your PDF text chunks`
    //     let sanitized_chunk = sanitize_chunk_comprehensive(chunk.as_str());
    //     // info!("i: {}, chunk: {}", i, sanitized_chunk);
    //     documents.push((
    //         sanitized_chunk,
    //         DocumentMetaData {
    //             index: i,
    //             source: moores_law_for_everything_pdf_file_path.to_string(),
    //         },
    //     ));
    // }

    // Add chunks from The Last Question
    // info!("chunking The Last Question");
    // for (i, chunk) in last_question_chunks.into_iter().enumerate() {
    //     // required to sanitize to prevent server crash with `NUL bytes (\0) in your PDF text chunks`
    //     let sanitized_chunk = sanitize_chunk_comprehensive(chunk.as_str());
    //     // info!("i: {}, chunk: {}", i, sanitized_chunk);
    //     documents.push((
    //         sanitized_chunk,
    //         DocumentMetaData {
    //             index: i,
    //             source: the_last_question_pdf_file_path.to_string(),
    //         },
    //     ));
    // }

    // TODO:
    // info!("documents: {:?}", serde_json::to_string_pretty(&documents)?);
    let mut file = std::fs::File::create("init_pdf_documents.json")?;
    serde_json::to_writer_pretty(&mut file, &documents)?;

    // Store documents in the knowledge base
    let doc_ids = rag.store_documents(documents).await?;
    info!("Stored documents with IDs: {:?}", doc_ids);

    Ok(())
}
