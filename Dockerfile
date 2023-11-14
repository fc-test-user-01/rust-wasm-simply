FROM rust:latest

RUN apt update
RUN apt install wget curl vim nmap -y
RUN cargo install wasm-bindgen-cli
COPY /src /workspace/src
WORKDIR /workspace
RUN cargo new test-wasm
WORKDIR /workspace/test-wasm
COPY /src/main.rs /workspace/test-wasm/src/
RUN rustup target add wasm32-unknown-unknown
RUN cargo build --target wasm32-unknown-unknown
RUN wasm-bindgen target/wasm32-unknown-unknown/debug/test-wasm.wasm --out-dir /workspace/dist
COPY /src/index.html /workspace/dist
WORKDIR /workspace
