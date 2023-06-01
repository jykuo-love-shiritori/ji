FROM node:alpine AS frontend
RUN npm install --global pnpm

WORKDIR /frontend
COPY frontend/package.json frontend/pnpm-lock.yaml ./
RUN pnpm install

COPY frontend ./
RUN pnpm build
RUN pnpm prune --prod

FROM rust:alpine AS backend
RUN apk add musl-dev openssl openssl-dev

RUN cargo new --bin backend
WORKDIR /backend

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/ji*
RUN cargo build --release

FROM alpine AS deploy
WORKDIR /app
COPY --from=frontend /frontend/dist ./frontend/dist
COPY --from=frontend /frontend/node_modules ./frontend/node_modules
COPY --from=backend /backend/target/release/ji .

EXPOSE 8000

CMD ["./ji"]
