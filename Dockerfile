# https://www.21analytics.ch/blog/docker-from-scratch-for-rust-applications/ and also lots of extra googling

# ---------- Stage 1: Build ----------
FROM rust:slim AS builder

ENV TARGET=x86_64-unknown-linux-musl
RUN rustup target add "$TARGET"

WORKDIR /app
COPY . .

RUN cargo build --release --locked --target "$TARGET"

CMD ["/app/app"]


# ---------- Stage 2: Minimal runtime ----------
FROM scratch
#
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/app /app/app

ENTRYPOINT ["/app/app"]
#CMD ["/app/app"]

