FROM busybox AS box
ARG BUSYBOX_VERSION=1.31.0-i686-uclibc


FROM alpine AS builder
ARG appName=""
COPY ./target/release/${appName} /
RUN mv /${appName} /app

FROM gcr.io/distroless/cc-debian12 AS run_env
COPY --from=builder /app /
COPY --from=box /bin/wget /usr/bin/wget

EXPOSE 3000
CMD ["./app"]
