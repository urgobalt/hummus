FROM alpine:3.21 AS builder
COPY . /app
WORKDIR /app

RUN --network=host apk add rustup pnpm just && \
    rustup show && \
    just build-server

FROM alpine:3.21
COPY --from=builder /app/target/release/server /app/ui/dist .
ENTRYPOINT [ "./server" ]
