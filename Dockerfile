FROM rust:alpine as builder
ENV PNPM_HOME="/pnpm"
RUN apk add --no-cache musl-dev build-base libc-dev libressl-dev nodejs npm
RUN wget -qO- https://get.pnpm.io/install.sh | ENV="$HOME/.shrc" SHELL="$(which sh)" sh -
ENV PATH="$PATH:$PNPM_HOME"
RUN npm install -g corepack
RUN corepack enable
RUN cargo install tauri-cli
COPY . /build
RUN cd /build && pnpm install && pnpm run build:web
RUN cd /build/server && cargo build --release

FROM alpine:latest as runner
COPY --from=builder /build/target/release/chat-wizard-server /app/chat-wizard-server
CMD ["/app/chat-wizard-server"]
