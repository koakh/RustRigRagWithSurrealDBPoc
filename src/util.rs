// https://claude.ai/chat/0ebcfe4b-4206-4999-b94d-af60407e40b2
use anyhow::Result;
use std::collections::HashMap;

use crate::rag_system::DocumentMetaData;

pub fn hashmap_to_json_value(hashmap: HashMap<String, String>) -> serde_json::Value {
    serde_json::to_value(hashmap).unwrap_or(serde_json::Value::Object(serde_json::Map::new()))
}

pub fn save_json_file(documents: &Vec<(String, DocumentMetaData)>) -> Result<()> {
    let mut file = std::fs::File::create("init_pdf_documents.json")?;
    serde_json::to_writer_pretty(&mut file, &documents)?;
    Ok(())
}
