# 'builder' stage installs all dependencies and builds the project
FROM rust:bookworm as builder
WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
# This will cache dependencies and perform a dummy build.
RUN cargo build --release

# Now copy actual source code and perform the real build
COPY src/ ./src
RUN cargo clean
RUN cargo build --release

# 'runtime' stage simply uses ready artifacts from 'builder' without any redundant build artefacts
FROM debian:bookworm-slim
WORKDIR /usr/local/bin

RUN apt-get update
RUN apt-get update && apt install -y openssl
RUN apt-get install -y libssl-dev   # <-- Add this line
COPY --from=builder /usr/src/app/target/release/rust-complete-webserver .

# This line exposes port 8080
EXPOSE 8080

# This allows application logs to be viewed using `docker logs`.
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*
CMD ["./rust-complete-webserver"]