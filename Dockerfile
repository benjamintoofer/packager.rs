# Builder stage
FROM rust:1.53.0 as rust-builder
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get upgrade -y && apt-get install -y build-essential git clang llvm-dev libclang-dev libssl-dev pkg-config libpq-dev musl-tools brotli

RUN USER=root cargo new packager
WORKDIR /usr/src/packager
COPY Cargo.toml Cargo.lock ./
RUN cargo build

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .
#RUN cargo build

# Runtime stage
FROM scratch AS runtime
# RUN apt-get update && apt-get install -y procps net-tools
WORKDIR /usr/local/app
# Copy the compiled binary from the builder environment 
# to our runtime environment
COPY --from=rust-builder /usr/local/cargo/bin/packager .
#USER 1000
CMD ["./packager"]