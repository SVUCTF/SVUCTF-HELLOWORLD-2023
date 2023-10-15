FROM rust:latest

RUN apt-get update && apt-get install -y \
    golang-go gcc

WORKDIR /app

COPY target/release/oj /app/oj
COPY config.json /app/config.json
COPY data/ /app/data/
COPY --chmod=500 init.sh /etc/init.sh

CMD ["/etc/init.sh"]
