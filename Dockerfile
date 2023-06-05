# https://andygrove.io/2020/05/why-musl-extremely-slow/
FROM node:slim AS frontend
RUN npm install --global pnpm

WORKDIR /app
COPY frontend/package.json frontend/pnpm-lock.yaml ./
RUN pnpm install

COPY frontend ./
RUN pnpm build
RUN pnpm prune --prod

FROM rust:slim AS backend
RUN apt-get -y update && apt-get -y install pkg-config libssl-dev

WORKDIR /app
RUN cargo init --bin .

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/ji*
RUN cargo build --release

FROM debian:bullseye-slim AS deploy
WORKDIR /app
COPY --from=frontend /app/dist ./frontend/dist
COPY --from=backend /app/target/release/ji .

EXPOSE 8000

CMD ["./ji"]
