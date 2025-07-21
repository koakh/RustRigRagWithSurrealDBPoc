use serde::Deserialize;

// Environment variables defaults
fn default_surreal_db_url() -> String {
  "127.0.0.1:8000".to_string()
}

fn default_surreal_db_user() -> String {
  "root".to_string()
}

fn default_surreal_db_pass() -> String {
  "root".to_string()
}

fn default_surreal_db_ns() -> String {
  "rag".to_string()
}

fn default_surreal_db_db() -> String {
  "documents".to_string()
}

fn default_ollama_url() -> String {
  "http://localhost:11434".to_string()
}

fn default_ollama_embedding_model() -> String {
  "nomic-embed-text".to_string()
}

fn default_ollama_generation_model() -> String {
  "llama3.2".to_string()
}

// Data structures for Environment variables
#[derive(Deserialize, Debug)]
pub struct Configuration {
  #[serde(default = "default_surreal_db_url")]
  pub surreal_db_url: String,
  #[serde(default = "default_surreal_db_user")]
  pub surreal_db_user: String,
  #[serde(default = "default_surreal_db_ns")]
  pub surreal_db_ns: String,
  #[serde(default = "default_surreal_db_db")]
  pub surreal_db_db: String,
  #[serde(default = "default_surreal_db_pass")]
  pub surreal_db_pass: String,
  #[serde(default = "default_ollama_url")]
  pub ollama_url: String,
  #[serde(default = "default_ollama_embedding_model")]
  pub ollama_embedding_model: String,
  #[serde(default = "default_ollama_generation_model")]
  pub ollama_generation_model: String,
}
