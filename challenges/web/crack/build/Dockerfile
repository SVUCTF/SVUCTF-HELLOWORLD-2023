FROM python:alpine

ENV FLASK_APP=app FLASK_ENV=production FLASK_ENV_DEBUG=False

RUN pip install flask gunicorn --no-cache-dir

COPY --chmod=500 init.sh /init.sh
COPY ./app /app

WORKDIR /app

CMD ["/init.sh"]
