use clap::Command;
use tracing::{error, info};

use crate::{info, init_schema, query, RagSystem};

pub struct Cli {}

impl Cli {
    pub async fn run(rag: &RagSystem) {
        let mut cmd = Command::new("rigrag")
            .version("1.0")
            .about("Rig Rag Rust PoC CLI")
            .subcommand(
                Command::new("init")
                    .about("Init RAG system: SurrealDB Schema and Sample Documents"),
            )
            .subcommand(Command::new("info").about("Get Knowledge base info"))
            .subcommand(Command::new("query").about("Query knowledge base sample documents"));

        let matches = cmd.clone().get_matches();

        match matches.subcommand() {
            Some(("init", _sub_matches)) => match init_schema(rag).await {
                Ok(_) => info!("Init RAG SurrealDB Schema and Sample Documents"),
                Err(e) => error!("{}", e),
            },
            Some(("info", _sub_matches)) => match info(rag).await {
                Ok(_) => info!("Knowledge base Info"),
                Err(e) => error!("{}", e),
            },
            Some(("query", _sub_matches)) => match query(rag).await {
                Ok(_) => info!("Query Knowledge base"),
                Err(e) => error!("{}", e),
            },
            _ => {
                // Print help if no subcommand is matched
                let _ = cmd.print_help();
                // Add newline after help
                println!();
            }
        }
    }
}
