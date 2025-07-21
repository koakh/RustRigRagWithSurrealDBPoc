# NOTES

nix-shell ~/.nixshell/nix-shell-rust-openssl.nix --run fish

cargo add rig-surrealdb

surrealdb = { workspace = true, features = ["protocol-ws", "kv-mem"] }

https://github.com/0xPlaygrounds/rig/blob/main/rig-core/examples/ollama_streaming.rs
https://github.com/0xPlaygrounds/rig/blob/main/rig-surrealdb/Cargo.toml

## Start Stack

```shell
$ mkdir volumes/surrealdb/
$ sudo chown 65532:65532 -R volumes/surrealdb/
$ docker-compose up -d
$ docker exec ollama ./ollama pull nomic-embed-text
$ docker exec ollama ./ollama pull llama3.2
$ docker exec ollama ./ollama list
```

## Run Project

```shell
$ cargo run
2025-07-21T10:02:10.416590Z  INFO rag_system: Starting RAG system with Ollama and SurrealDB
2025-07-21T10:02:10.632237Z  INFO rag_system: RAG System initialized successfully
2025-07-21T10:02:10.643694Z  INFO rag_system: Database schema initialized
Error: There was a problem with the database: Incorrect vector dimension (768). Expected a vector of 384 dimension.

Caused by:
    There was a problem with the database: Incorrect vector dimension (768). Expected a vector of 384 dimension.
```

fix: change `DEFINE INDEX embedding_idx ON documents FIELDS embedding MTREE DIMENSION 384;` to `DEFINE INDEX embedding_idx ON documents FIELDS embedding MTREE DIMENSION 768;`