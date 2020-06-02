FROM rust:1.40

WORKDIR /app

COPY . /app

RUN cargo build

EXPOSE 8080