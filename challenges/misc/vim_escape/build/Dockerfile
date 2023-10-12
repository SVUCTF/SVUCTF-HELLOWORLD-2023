FROM tsl0922/ttyd:latest

RUN echo $GZCTF_FLAG > /root/flag && unset GZCTF_FLAG && \
    apt-get update -y && apt-get install vim man-db -y && unminimize

COPY vimrc /root/.vimrc
COPY vimrc /root/.vimrc.bak

CMD ["ttyd", "-W", "vim", "/root/.vimrc.bak"]