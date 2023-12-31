FROM ubuntu:22.04 AS builder
FROM ghcr.io/svuctf/base/xinetd:alpine

COPY init.sh /init.sh
COPY xinetd.conf /etc/xinetd.conf

RUN chmod +x /init.sh && \
    chown -R ctf:ctf /home/ctf && \
    chmod -R 750 /home/ctf && \
    cp -R /lib* /home/ctf && \
    mkdir /home/ctf/lib64  && \
    mkdir /home/ctf/dev  && \
    mknod /home/ctf/dev/null c 1 3 && \
    mknod /home/ctf/dev/zero c 1 5 && \
    mknod /home/ctf/dev/random c 1 8 && \
    mknod /home/ctf/dev/urandom c 1 9 && \
    chmod 666 /home/ctf/dev/* && \
    mkdir /home/ctf/bin && \
    mkdir -p /home/ctf/lib/x86_64-linux-gnu/ && \
    mkdir -p /home/ctf/lib32/ && \
    cp /bin/sh /home/ctf/bin && \
    cp /bin/ls /home/ctf/bin && \
    cp /bin/cat /home/ctf/bin && \
    mkdir /home/ctf/.secret && \
    cp /usr/bin/id /home/ctf/bin && \
    apk add coreutils --no-cache

COPY --from=builder /lib/x86_64-linux-gnu/libc.so.6 /home/ctf/lib/x86_64-linux-gnu/
COPY --from=builder /lib/x86_64-linux-gnu/ld-linux-x86-64.so.2 /home/ctf/lib64/
COPY --chown=ctf:ctf --chmod=400 myapp.c /home/ctf/.secret/myapp.c
COPY --chown=root:root myapp /home/ctf/myapp
RUN chmod +s /home/ctf/myapp

CMD ["xinetd", "-dontfork"]
