FROM rust:latest

WORKDIR /usr/app

# Workaround caching deps
RUN echo 'fn main() {}' > dummy.rs
COPY Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build

RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

COPY . .

RUN cargo build

# Rebuild on change in src
RUN cargo install cargo-watch