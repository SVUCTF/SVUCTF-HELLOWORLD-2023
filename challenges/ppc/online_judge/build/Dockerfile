FROM golang:latest as go-builder
FROM gcc:latest as gcc-builder
FROM rust:1.73

WORKDIR /app

COPY target/release/oj /app/oj
COPY config.json /app/config.json
COPY data/ /app/data/
COPY --chmod=500 init.sh /etc/init.sh

COPY --from=go-builder /usr/local/go /usr/local/go
COPY --from=gcc-builder /usr/local /usr/local

ENV PATH="/usr/local/go/bin:$PATH"

CMD ["/etc/init.sh"]
