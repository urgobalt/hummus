FROM base AS builder
COPY . /app
WORKDIR /app

RUN just build-server

FROM scratch
COPY --from=builder /app/target/release/server /app/ui/dist /app

ENV PORT=80
ENV DIST=/app/dist

ENTRYPOINT [ "/app/server" ]
