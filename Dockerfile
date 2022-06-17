FROM rust:1.61-bullseye

WORKDIR /usr/src/rust_app_with_db
COPY . .

RUN cargo install --path .

CMD ["rust_app_with_db"]
