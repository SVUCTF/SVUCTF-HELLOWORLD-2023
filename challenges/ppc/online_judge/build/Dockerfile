FROM alpine:latest

RUN apk add --no-cache gcc g++ musl-dev

WORKDIR /app

COPY target/release/oj /app/oj
COPY config.json /app/config.json
COPY data/ /app/data/
COPY --chmod=500 init.sh /etc/init.sh

CMD ["/etc/init.sh"]
