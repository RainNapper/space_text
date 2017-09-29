FROM rust:latest
EXPOSE 8080
COPY Cargo.toml /
COPY Cargo.lock /
COPY src/main.rs /src/
CMD cargo run
