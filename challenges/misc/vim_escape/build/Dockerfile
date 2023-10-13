FROM tsl0922/ttyd:latest

RUN apt-get update -y && apt-get install vim man-db -y && yes | unminimize

COPY --chmod=500 init.sh /etc/init.sh
COPY --chmod=400 vimrc /root/.vimrc

CMD ["/etc/init.sh"]