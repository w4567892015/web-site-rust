FROM busybox AS box
ARG BUSYBOX_VERSION=1.31.0-i686-uclibc

FROM rust:latest AS base
ARG appName=""
WORKDIR /usr/src/
COPY Cargo.toml Cargo.lock ./
COPY ./apps/${appName} ./apps/${appName}
COPY ./libs ./libs

FROM base AS builder
RUN <<EOF
  cargo build --release -p ${appName}
  mv target/release/${appName} target/release/main
  rm -rf target/release/deps target/release/.fingerprint
EOF

FROM gcr.io/distroless/cc-debian12
ARG appName=""
COPY --from=box /bin/wget /usr/bin/wget
COPY --from=builder /usr/src/target/release /app

WORKDIR /app
EXPOSE 3000
CMD ["./main"]
