FROM base AS builder
COPY . /app
WORKDIR /app

RUN just build-server

FROM scratch

COPY --from=builder /app/target/release/server /app/ui/dist /

ENV PORT=80
ENV DIST=/dist

ENTRYPOINT [ "/server" ]
