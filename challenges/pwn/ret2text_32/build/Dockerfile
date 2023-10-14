FROM i386/ubuntu:bionic AS builder

COPY ret2text_32.c /ret2text_32.c
RUN apt-get update -y && apt install gcc-4.8 -y && \
    gcc-4.8 -o ret2text_32 ret2text_32.c -fno-stack-protector -z lazy

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
    mkdir -p /home/ctf/lib/i386-linux-gnu/ && \
    mkdir -p /home/ctf/lib32/ && \
    cp /bin/sh /home/ctf/bin && \
    cp /bin/ls /home/ctf/bin && \
    cp /bin/cat /home/ctf/bin && \
    cp /bin/base64 /home/ctf/bin

COPY --from=builder /lib/i386-linux-gnu/libc.so.6 /home/ctf/lib/i386-linux-gnu/
COPY --from=builder /lib/ld-linux.so.2 /home/ctf/lib/
COPY --from=builder --chown=ctf:ctf --chmod=500 /ret2text_32 /home/ctf/ret2text_32

CMD ["xinetd", "-dontfork"]
