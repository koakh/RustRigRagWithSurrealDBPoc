# NOTES

## BootStrap NixEnv

```shell
$ nix-shell ~/.nixshell/nix-shell-rust-openssl.nix --run fish
```

## Add SurrealDb

```shell
$ cargo add rig-surrealdb
```

```toml
[dependencies]
surrealdb = { workspace = true, features = ["protocol-ws", "kv-mem"] }
```

## Used Startup Examples

- <https://github.com/0xPlaygrounds/rig/blob/main/rig-core/examples/ollama_streaming.rs>
- <https://github.com/0xPlaygrounds/rig/blob/main/rig-surrealdb/Cargo.toml>

## Used ChatBot Links

- [Just a moment...](https://claude.ai/chat/ec7135ae-573a-466c-b18b-96b69ba1694a)
- [Just a moment...](https://claude.ai/chat/f6808d1b-8fb5-454a-b4de-666959834bb8)

## Start Stack

```shell
$ sudo rm -rf volumes/surrealdb/data
$ sudo mkdir -p volumes/surrealdb/data
$ sudo chown 65532:65532 -R volumes/surrealdb/
$ sudo chmod 755 -R volumes/surrealdb/
# lift stack
$ docker-compose up -d
# pull models
$ docker exec ollama ./ollama pull nomic-embed-text
$ docker exec ollama ./ollama pull bge-m3:567m
$ docker exec ollama ./ollama pull llama3.2
# list models
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

## When Change to Other LLM Server Query errors

in machines

- `APP_OLLAMA_URL=http://192.168.90.104:11434`
- `APP_OLLAMA_URL=http://192.168.90.133:11434`

### Problem #1 : ollamagpu ollama[2874430]: decode: cannot decode batches with this context (use llama_encode() instead)

WIP:

## Examples

### PDF-Based RAG System with Rig

- [Build a RAG System with Rig in Under 100 Lines of Code](https://dev.to/0thtachi/build-a-rag-system-with-rig-in-under-100-lines-of-code-4422)
- [GitHub - 0xPlaygrounds/rig-rag-system-example](https://github.com/0xPlaygrounds/rig-rag-system-example)

required `features = ["pdf" ]` ex `rig-core = { version = "0.15.1", features = ["pdf"] }`
