FROM ghcr.io/svuctf/base/xinetd:alpine

COPY vimrc /root/.vimrc
COPY init.sh /init.sh
COPY xinetd.conf /etc/xinetd.conf
COPY repositories /etc/apk/repositories

RUN apk add vim --no-cache && chmod +x /init.sh

CMD ["xinetd", "-dontfork"]