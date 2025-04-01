FROM base AS builder
COPY . /app
WORKDIR /app

RUN just build-ids

FROM scratch
COPY --from=builder /app/target/release/ids /
ENTRYPOINT ["/ids"]
