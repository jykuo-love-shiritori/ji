# https://andygrove.io/2020/05/why-musl-extremely-slow/
FROM node:slim AS frontend
RUN npm install --global pnpm

WORKDIR /frontend
COPY frontend/package.json frontend/pnpm-lock.yaml ./
RUN pnpm install

COPY frontend ./
RUN pnpm build
RUN pnpm prune --prod

FROM rust:slim AS backend
RUN apt-get -y update && apt-get -y install pkg-config libssl-dev

RUN cargo new --bin backend
WORKDIR /backend

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/ji*
RUN cargo build --release

FROM debian:bullseye-slim AS deploy
WORKDIR /app
COPY --from=frontend /frontend/dist ./frontend/dist
COPY --from=frontend /frontend/node_modules ./frontend/node_modules
COPY --from=backend /backend/target/release/ji .

EXPOSE 8000

CMD ["./ji"]
