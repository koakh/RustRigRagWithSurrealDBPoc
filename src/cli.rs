use clap::Command;
use tracing::{ error, info };

use crate::{ info, init_documents, init_pdf_documents, init_schema, query, RagSystem };

pub struct Cli {}

impl Cli {
    pub async fn run(rag: &RagSystem) {
        let mut cmd = Command::new("rigrag")
            .version("1.0")
            .about("Rig Rag Rust PoC CLI")
            .subcommand(Command::new("info").about("Knowledge base info"))
            .subcommand(Command::new("init-schema").about("Init vectorDB Schema"))
            .subcommand(Command::new("init-documents").about("Init sample documents"))
            .subcommand(Command::new("init-pdf-documents").about("Init PDF sample documents"))
            .subcommand(Command::new("query").about("Query knowledge base sample documents"));

        let matches = cmd.clone().get_matches();

        match matches.subcommand() {
            Some(("info", _sub_matches)) =>
                match info(rag).await {
                    Ok(_) => info!("Finished knowledge base info"),
                    Err(e) => error!("{}", e),
                }
            Some(("init-schema", _sub_matches)) =>
                match init_schema(rag).await {
                    Ok(_) => info!("Finished vectorDB schema initialization"),
                    Err(e) => error!("{}", e),
                }
            Some(("init-documents", _sub_matches)) =>
                match init_documents(rag).await {
                    Ok(_) => info!("Finished sample documents initialization"),
                    Err(e) => error!("{}", e),
                }
            Some(("init-pdf-documents", _sub_matches)) =>
                match init_pdf_documents(rag).await {
                    Ok(_) => info!("Finished PDF sample documents initialization"),
                    Err(e) => error!("{}", e),
                }
            Some(("query", _sub_matches)) =>
                match query(rag).await {
                    Ok(_) => info!("Finished knowledge base sample documents query"),
                    Err(e) => error!("{}", e),
                }
            _ => {
                // Print help if no subcommand is matched
                let _ = cmd.print_help();
                // Add newline after help
                println!();
            }
        }
    }
}
