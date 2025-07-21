# NOTES

nix-shell ~/.nixshell/nix-shell-rust-openssl.nix --run fish

cargo add rig-surrealdb

surrealdb = { workspace = true, features = ["protocol-ws", "kv-mem"] }

https://github.com/0xPlaygrounds/rig/blob/main/rig-core/examples/ollama_streaming.rs
https://github.com/0xPlaygrounds/rig/blob/main/rig-surrealdb/Cargo.toml

sudo chown 65532:65532 -R volumes/surrealdb/