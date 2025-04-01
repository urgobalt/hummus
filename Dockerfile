FROM alpine:latest

ENV PATH=${PATH}:/root/.cargo/bin

RUN apk update && \
    apk add coreutils curl bash pnpm gcc musl-dev --no-cache  && \
    apk add rustup just --no-cache --repository=https://dl-cdn.alpinelinux.org/alpine/edge/community

RUN rustup-init --default-toolchain none -y
RUN rustup toolchain install nightly \
    --profile minimal \
    --component rustc,rust-std,cargo && \
    rustup default nightly

