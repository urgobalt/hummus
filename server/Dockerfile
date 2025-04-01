FROM alpine:latest AS builder
COPY . /app
WORKDIR /app

RUN apk update && \
    apk add coreutils curl bash pnpm gcc musl-dev --no-cache  && \
    apk add rustup just --no-cache --repository=https://dl-cdn.alpinelinux.org/alpine/edge/community

RUN rustup-init --default-toolchain none -y
RUN export PATH=$HOME/.cargo/bin:$PATH && \
    rustup toolchain install nightly \
    --profile minimal \
    --component rustc,rust-std,cargo && \
    rustup default nightly
RUN PATH=$HOME/.cargo/bin:$PATH just build-server

FROM scratch
COPY --from=builder /app/target/release/server /app/ui/dist .
ENV PORT=80
ENTRYPOINT [ "./server" ]
