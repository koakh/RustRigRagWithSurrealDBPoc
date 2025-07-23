LOG_DEFAULT_LEVEL := DEBUG;
SHELL := /bin/bash

env:
	@nix-shell ~/.nixshell/nix-shell-rust-openssl.nix --run fish

build:
	@cargo build

buildRelease:
	@cargo build --release

start:
	@cargo build && \
  HTTP_SERVER_URI=0.0.0.0:$(REACT_APP_PORT_WS) \
	cargo run -- start-server

startDebug:
	@cargo build && \
    echo 0 | sudo tee /proc/sys/kernel/yama/ptrace_scope \
		&& target/debug/rag_system start-server