FROM rust:alpine AS build
RUN apk add musl-dev

RUN cargo new --bin backend
WORKDIR /backend

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/backend*
RUN cargo build --release

FROM alpine AS deploy
WORKDIR /app
COPY --from=build /backend/target/release/backend .

CMD ["./backend"]
