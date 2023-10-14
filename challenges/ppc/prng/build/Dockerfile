FROM ghcr.io/svuctf/base/python:alpine

COPY --chmod=775 main.py /app/main.py
COPY xinetd.conf /etc/xinetd.conf

CMD ["xinetd", "-dontfork"]
